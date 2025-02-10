use axum::{Extension, body::Body, debug_handler, extract::Request, response::Response};
use hyper::{StatusCode, header::CONTENT_TYPE};
use std::{fs, path::PathBuf};

#[debug_handler]
pub async fn handle_embedded(
    Extension(dir): Extension<PathBuf>,
    req: Request<Body>,
) -> Response<Body> {
    let path = req.uri().path();
    let index_path = format!("{}index.html", path);

    let path = if path.ends_with('/') {
        index_path.as_str()
    } else {
        path
    };

    let path = path.trim_start_matches('/');
    let file_path = dir.join(path);
    let fallback_path = dir.join("fallback.html");
    let not_found_path = dir.join("404.html");
    let spa_path = dir.join("index.html");

    // Try the file
    if file_path.exists() {
        let mime = mime_guess::from_path(path).first().unwrap();

        return Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, mime.to_string())
            .body(Body::from(fs::read(file_path).unwrap()))
            .unwrap();
    }

    // Try a fallback page
    if fallback_path.exists() {
        return Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(fs::read(fallback_path).unwrap()))
            .unwrap();
    }

    // Try a 404 page
    if not_found_path.exists() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(fs::read(not_found_path).unwrap()))
            .unwrap();
    }

    // Maybe it's an SPA?
    if spa_path.exists() {
        return Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(fs::read(spa_path).unwrap()))
            .unwrap();
    }

    // 404 not found in plaintext
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(
            "Cannot find the file specified!".as_bytes().to_vec(),
        ))
        .unwrap()
}
