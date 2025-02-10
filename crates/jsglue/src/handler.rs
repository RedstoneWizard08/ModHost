use crate::{
    query::parse_query,
    runner::CLIENT_READY,
    state::ProxyState,
    util::{req_to_bytes, res_to_bytes},
};
use axum::{Extension, body::Body, debug_handler, extract::Request, response::Response};
use hyper::{
    StatusCode,
    header::{CONTENT_LENGTH, CONTENT_TYPE, TRANSFER_ENCODING},
};

#[debug_handler]
pub async fn fallback_handler(
    Extension(state): Extension<ProxyState>,
    req: Request<Body>,
) -> Response<Body> {
    unsafe {
        if !CLIENT_READY {
            return Response::builder()
                .status(StatusCode::SERVICE_UNAVAILABLE)
                .header(CONTENT_TYPE, "text/plain")
                .body(Body::from("Frontend is starting..."))
                .unwrap();
        }
    }

    let method = req.method().clone();
    let path = req.uri().clone();
    let path = path.path();
    let uri = req.uri().clone();
    let headers = req.headers().clone();

    let res = state
        .request(
            method,
            path,
            uri.query().map(parse_query),
            Some(req_to_bytes(req).await),
            Some(headers),
        )
        .await
        .unwrap();

    let mut builder = Response::builder().status(res.status());

    for key in res.headers().keys() {
        if *key == CONTENT_LENGTH || *key == TRANSFER_ENCODING {
            continue;
        }

        builder = builder.header(key, res.headers().get(key).unwrap());
    }

    builder.body(Body::from(res_to_bytes(res).await)).unwrap()
}
