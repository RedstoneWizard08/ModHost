//! ModHost's OpenAPI system using [`utoipa`].

use crate::{
    auth::AuthApi, meta::MetadataApi, moderation::ModerationApi, projects::ProjectsApi,
    users::UsersApi,
};
use modhost_config::AppConfig;
use utoipa::{
    openapi::{
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        tag::TagBuilder,
        InfoBuilder, LicenseBuilder, OpenApi, OpenApiBuilder, Tag,
    },
    Modify, OpenApi as OpenApiTrait,
};

/// An addon for the OpenAPI spec to add token auth.
#[derive(Debug, Clone, Copy)]
pub struct TokenAuthAddon;

impl Modify for TokenAuthAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "api_auth_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("TOKEN")
                    .build(),
            ),
        )
    }
}

/// A utility macro to apply types to a new [`utoipa::openapi::schema::ComponentBuilder`]
/// from a bunch of crates.
macro_rules! apply_types {
    [$($parent: ident),* $(,)?] => {{
        let mut components = utoipa::openapi::schema::ComponentsBuilder::new();

        $(
            components = $parent::add_types(components);
        )*

        components
    }};
}

/// Shorthand helper for creating an OpenAPI [`Tag`] from a name and a description.
pub fn make_tag(name: impl AsRef<str>, desc: impl AsRef<str>) -> Tag {
    TagBuilder::new()
        .name(name.as_ref())
        .description(Some(desc.as_ref()))
        .build()
}

/// Build the OpenAPI spec.
pub fn build_openapi(_config: &AppConfig) -> OpenApi {
    let tags = vec![
        make_tag("Auth", "Authentication endpoints."),
        make_tag("Users", "User-related endpoints."),
        make_tag("Projects", "Project-related endpoints."),
        make_tag("Gallery", "Project gallery-related endpoints."),
        make_tag("Versions", "Project version-related endpoints."),
        make_tag("Misc", "Miscellaneous endpoints."),
        make_tag("Meta", "Metadata-related endpoints."),
        make_tag("Moderation", "Moderation-related endpoints."),
    ];

    let components = apply_types![
        crate,
        modhost_db,
        modhost_search,
        modhost_server_core,
        modhost_badges,
    ]
    .build();

    let mut api = OpenApiBuilder::new()
        .info(
            InfoBuilder::new()
                .title("ModHost API")
                .description(Some("The ModHost REST API."))
                .license(Some(
                    LicenseBuilder::new()
                        .name("MIT")
                        .url(Some("https://opensource.org/license/mit/"))
                        .build(),
                ))
                .build(),
        )
        .tags(Some(tags))
        .components(Some(components))
        .build();

    TokenAuthAddon.modify(&mut api);

    api.nest("/api/v1/auth", AuthApi::openapi())
        .nest("/api/v1/meta", MetadataApi::openapi())
        .nest("/api/v1/projects", ProjectsApi::openapi())
        .nest("/api/v1/users", UsersApi::openapi())
        .nest("/api/v1/moderation", ModerationApi::openapi())
}
