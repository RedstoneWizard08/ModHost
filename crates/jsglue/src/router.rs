use crate::{
    embedded::handle_embedded, framework::Framework, handler::fallback_handler, state::ProxyState,
    ws::route::websocket_handler,
};
use axum::{Extension, Router, routing::get};
use std::path::PathBuf;

/// Register the proxy handler.
/// Accepts a base and a router.
/// The base must be in the format "http(s)://[ip or domain]:[optional port]"
/// with NO trailing slash (or it will break).
pub fn register_proxy<T>(base: String, router: Router<T>, framework: Option<Framework>) -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    let state = ProxyState::new(base, framework.unwrap_or(Framework::None));
    let mut router = router;

    if let Some(framework) = framework {
        router = router.route(framework.get_hmr_path(), get(websocket_handler));
    }

    router.fallback(fallback_handler).layer(Extension(state))
}

pub fn register_embedded<T>(dir: PathBuf, router: Router<T>) -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    router.fallback(handle_embedded).layer(Extension(dir))
}
