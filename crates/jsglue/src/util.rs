use axum::{body::Body, extract::Request};
use http_body_util::BodyExt;
use hyper::{
    Response,
    body::{Bytes, Incoming},
};

pub fn is_debug() -> bool {
    cfg_if! {
        if #[cfg(debug_assertions)] {
            true
        } else {
            false
        }
    }
}

pub async fn req_to_bytes(mut body: Request<Body>) -> Bytes {
    let mut bytes = Vec::new();

    while let Some(Ok(frame)) = body.frame().await {
        if let Some(chunk) = frame.data_ref() {
            for byte in chunk {
                bytes.push(*byte);
            }
        }
    }

    Bytes::from_iter(bytes)
}

pub async fn res_to_bytes(mut body: Response<Incoming>) -> Bytes {
    let mut bytes = Vec::new();

    while let Some(Ok(frame)) = body.frame().await {
        if let Some(chunk) = frame.data_ref() {
            for byte in chunk {
                bytes.push(*byte);
            }
        }
    }

    Bytes::from_iter(bytes)
}

pub fn scheme_port(scheme: &str) -> u16 {
    if scheme.starts_with("https") { 443 } else { 80 }
}
