use axumite::socket::WebSocket;
use futures_util::StreamExt;
use hyper::{
    Request, Uri, Version,
    header::{
        CONNECTION, HOST, SEC_WEBSOCKET_KEY, SEC_WEBSOCKET_PROTOCOL, SEC_WEBSOCKET_VERSION, UPGRADE,
    },
};
use std::net::SocketAddr;
use tokio::select;
use tokio_tungstenite::connect_async;
use tungstenite::handshake::client::generate_key;
use url::Url;

use crate::{state::ProxyState, ws::process::process_message};

pub async fn handle_websocket(state: ProxyState, uri: Uri, socket: WebSocket, who: SocketAddr) {
    let framework = state.framework;
    let url = Url::parse(&state.base).unwrap();
    let base = format!("{}/{}", url.authority(), url.path());
    let base = base.trim_end_matches('/');

    let proto = if url.scheme().starts_with("https") || url.scheme().starts_with("wss") {
        "wss"
    } else {
        "ws"
    };

    let uri = Uri::builder()
        .scheme(proto)
        .authority(base)
        .path_and_query(uri.path())
        .build()
        .unwrap();

    let mut req = Request::builder()
        .uri(uri.clone())
        .version(Version::HTTP_11)
        .header(HOST, uri.host().unwrap())
        .header(SEC_WEBSOCKET_KEY, generate_key())
        .header(SEC_WEBSOCKET_VERSION, 13)
        .header(CONNECTION, "Upgrade")
        .header(UPGRADE, "websocket");

    if let Some(proto) = framework.get_subprotocol() {
        req = req.header(SEC_WEBSOCKET_PROTOCOL, proto);
    }

    let req = req.body(()).unwrap();
    let res = connect_async(req).await;

    if let Err(err) = res {
        error!("Could not connect to proxy socket: {:?}", err);
        return;
    }

    let (psocket, _) = res.unwrap();
    let (mut psend, mut precv) = psocket.split();
    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        let mut cnt = 0;

        while let Some(Ok(msg)) = precv.next().await {
            cnt += 1;

            if process_message(&mut sender, who, msg).await.is_break() {
                break;
            }
        }

        cnt
    });

    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;

        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;

            if process_message(&mut psend, who, msg).await.is_break() {
                break;
            }
        }

        cnt
    });

    select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => info!("{a} messages sent to {who}"),
                Err(a) => error!("Error sending messages {a:?}"),
            };

            recv_task.abort();
        }

        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => info!("Received {b} messages"),
                Err(b) => error!("Error receiving messages {b:?}"),
            }

            send_task.abort();
        }
    }

    info!("Websocket context {} destroyed!", who);
}
