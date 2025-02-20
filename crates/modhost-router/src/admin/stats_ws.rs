//! The stats websocket route.

use crate::util::stats::{AdminStats, STATS_CHANNEL, fetch_stats};
use axum::{
    extract::{
        Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use modhost_core::{AppError, Result};
use modhost_db::get_user_for_token;
use modhost_server_core::state::AppState;

/// Query params for the admin stats Websocket.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct AdminStatsSocketQueryParams {
    /// Your token.
    pub t: String,
}

/// Stats (WebSocket)
///
/// Get statistics about this ModHost instance every [`crate::util::stats::STATS_INTERVAL`].
/// Tokens must be sent in the `?t` query parameter.
#[utoipa::path(
    get,
    path = "/stats/ws",
    tag = "Admin",
    responses(
        (status = 200, description = "Got stats!", body = AdminStats),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("t" = String, Query, description = "Your auth token."),
    ),
)]
#[debug_handler]
pub async fn stats_socket_handler(
    Query(AdminStatsSocketQueryParams { t }): Query<AdminStatsSocketQueryParams>,
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_for_token(t, &mut conn)
        .await?
        .ok_or(AppError::InvalidToken)?;

    if !user.admin {
        return Err(AppError::NoAccess);
    }

    Ok(ws.on_upgrade(async |s| handle_stats_socket(s, state).await.unwrap()))
}

/// The actual stats socket handler itself.
pub async fn handle_stats_socket(socket: WebSocket, state: AppState) -> Result<()> {
    info!("Got admin stats socket connection!");

    let (mut tx, mut rx) = socket.split();

    let mut writer = tokio::spawn(async move {
        tx.send(Message::Text(
            serde_json::to_string(
                &fetch_stats(
                    &state.buckets.projects,
                    &state.buckets.gallery,
                    &state.search.projects(),
                    &mut state.pool.get().await?,
                )
                .await?,
            )?
            .into(),
        ))
        .await?;

        let mut receiver = STATS_CHANNEL.0.subscribe();

        while let Ok(stats) = receiver.recv().await {
            tx.send(Message::Text(serde_json::to_string(&stats)?.into()))
                .await?;
        }

        Ok(()) as Result<()>
    });

    let mut reader = tokio::spawn(async move {
        while let Some(msg) = rx.next().await {
            if let Ok(msg) = msg {
                if let Message::Close(close) = msg {
                    info!("Closing admin stats socket: {:?}", close);
                    break;
                }
            } else {
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut writer) => { reader.abort(); }
        _ = (&mut reader) => { writer.abort(); }
    }

    info!("Exited admin stats socket handler!");

    Ok(())
}
