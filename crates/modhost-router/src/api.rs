//! OpenAPI routes.

use axum::{
    Router,
    extract::{Query, State},
    routing::get,
};
use modhost_core::Result;
use modhost_server_core::state::AppState;
use utoipa::openapi::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable as ServableRedoc};
use utoipa_scalar::{Scalar, Servable as ServableScalar};
use utoipa_swagger_ui::{Config, SwaggerUi};

/// Parameters for the JSON OpenAPI format.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ToSchema, ToResponse, Serialize, Deserialize,
)]
pub struct JsonQueryParams {
    /// Should it be pretty-printed?
    pub pretty: Option<bool>,
}

/// OpenAPI YAML
///
/// Get the YAML version of the OpenAPI definition.
#[utoipa::path(
    get,
    path = "/api/v1/openapi/yaml",
    tag = "Misc",
    responses(
        (status = 200, description = "The OpenAPI spec.", body = String),
    ),
)]
#[debug_handler]
pub async fn yaml_api(State(state): State<AppState>) -> Result<String> {
    Ok(state.api_spec.to_yaml()?)
}

/// OpenAPI JSON
///
/// Get the JSON version of the OpenAPI definition.
#[utoipa::path(
    get,
    path = "/api/v1/openapi/json",
    tag = "Misc",
    params(
        ("pretty" = bool, Query, description = "Should the JSON be pretty-printed?"),
    ),
    responses(
        (status = 200, description = "The OpenAPI spec.", body = String),
    ),
)]
#[debug_handler]
pub async fn json_api(
    State(state): State<AppState>,
    Query(JsonQueryParams { pretty }): Query<JsonQueryParams>,
) -> Result<String> {
    if pretty.unwrap_or(false) {
        Ok(state.api_spec.to_pretty_json()?)
    } else {
        Ok(state.api_spec.to_json()?)
    }
}

/// Register API docs and spec routes.
pub fn register(api_spec: &OpenApi, router: Router<AppState>) -> Router<AppState> {
    router
        .merge(
            SwaggerUi::new("/api/v1/docs/swagger")
                .config(Config::default().try_it_out_enabled(true))
                .url("/api/v1/docs/_swagger", api_spec.clone()),
        )
        .merge(
            Redoc::with_url("/api/v1/docs/redoc", api_spec.clone())
                .custom_html(include_str!("./redoc.html")),
        )
        .merge(Scalar::with_url("/api/v1/docs/scalar", api_spec.clone()))
        .merge(RapiDoc::new("/api/v1/openapi/json").path("/api/v1/docs/rapidoc"))
        .route("/api/v1/openapi/yaml", get(yaml_api))
        .route("/api/v1/openapi/json", get(json_api))
}
