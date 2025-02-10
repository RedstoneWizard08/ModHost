use std::net::SocketAddr;

use axum::{
    Extension,
    body::Body,
    debug_handler,
    extract::{ConnectInfo, Request},
    response::IntoResponse,
};
use axumite::upgrade::WebSocketUpgrade;
use hyper::{
    HeaderMap,
    header::{SEC_WEBSOCKET_PROTOCOL, USER_AGENT},
};
use tungstenite::http::HeaderValue;

use crate::{state::ProxyState, ws::handler::handle_websocket};

#[debug_handler]
pub async fn websocket_handler(
    Extension(state): Extension<ProxyState>,
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
) -> impl IntoResponse {
    let user_agent = headers
        .get(USER_AGENT)
        .map(|v| v.to_str().unwrap())
        .unwrap_or("Unknown browser");

    debug!("{} connected. (UA: {})", addr, user_agent);

    let cloned_state = state.clone();

    let mut res = ws
        .on_upgrade(move |socket| handle_websocket(cloned_state, req.uri().clone(), socket, addr));

    if let Some(proto) = state.framework.get_subprotocol() {
        res.headers_mut().insert(
            SEC_WEBSOCKET_PROTOCOL,
            HeaderValue::from_str(proto).unwrap(),
        );
    }

    res
}
