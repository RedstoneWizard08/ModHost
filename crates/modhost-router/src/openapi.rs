//! ModHost's OpenAPI system using [`utoipa`].

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};

/// The struct for the generated OpenAPI spec using [`utoipa`].
#[derive(OpenApi)]
#[openapi(
    info(
        title = "ModHost API",
        description = "The ModHost REST API.",

        license(
            name = "MIT",
            url = "https://opensource.org/license/mit/",
        ),
    ),
    paths(
        crate::api::yaml_api,
        crate::api::json_api,
        crate::users::me::me_handler,
        crate::users::info::info_handler,
        crate::users::pkg::list_handler,
        crate::auth::login::login_handler,
        crate::auth::callback::callback_handler,
        crate::pkg::info::project_info_handler,
        crate::pkg::info::update_project_handler,
        crate::pkg::info::delete_project_handler,
        crate::pkg::info::create_project_handler,
        crate::pkg::ver::list_versions_handler,
        crate::pkg::ver::version_info_handler,
        crate::pkg::ver::download_version_handler,
        crate::pkg::ver::create_version_handler,
        crate::pkg::ver::update_version_handler,
        crate::pkg::ver::delete_version_handler,
        crate::pkg::ver::latest_version_handler,
        crate::pkg::author::list_authors_handler,
        crate::pkg::author::add_author_handler,
        crate::pkg::author::remove_author_handler,
        crate::pkg::search::search_projects_handler,
        crate::pkg::gallery::list_gallery_handler,
        crate::pkg::gallery::upload_gallery_handler,
        crate::pkg::gallery::update_gallery_handler,
        crate::pkg::gallery::delete_gallery_handler,
        crate::pkg::gallery::s3_image_handler,
        crate::pkg::gallery::gallery_info_handler,
        crate::meta::badge::version_handler,
        crate::meta::badge::latest_version_badge_handler,
        crate::meta::vers::game_versions_handler,
        crate::meta::loaders::loaders_handler,
        crate::meta::tags::tags_handler,
    ),
    components(
        schemas(
            modhost_db::User,
            modhost_db::UserToken,
            modhost_db::NewUser,
            modhost_db::NewUserToken,
            modhost_db::ProjectManifest,
            modhost_db::Project,
            modhost_db::ProjectAuthor,
            modhost_db::ProjectRelation,
            modhost_db::ProjectVersion,
            modhost_db::ProjectVersionRef,
            modhost_db::ProjectVersionInit,
            modhost_db::NewProject,
            modhost_db::NewProjectVersion,
            modhost_db::RelationKind,
            modhost_db::ProjectData,
            modhost_db::ProjectVisibility,
            modhost_db::GalleryImage,
            modhost_db::NewGalleryImage,
            modhost_db::PublicGalleryImage,
            modhost_db::ProjectFile,
            modhost_db::NewProjectFile,
            modhost_db::ProjectVersionData,
            modhost_search::Sort,
            modhost_search::SortMode,
            modhost_search::SearchResults,
            modhost_search::Facet,
            crate::api::JsonQueryParams,
            crate::pkg::info::PartialProject,
            crate::pkg::ver::PartialProjectVersion,
            crate::pkg::search::SearchQuery,
            crate::pkg::gallery::PartialGalleryImage,
            crate::pkg::gallery::GalleryImageUpload,
            modhost_server_core::models::GameVersion,
            modhost_server_core::models::ModLoader,
            modhost_server_core::models::Tag,
        ),
        responses(
            modhost_db::User,
            modhost_db::UserToken,
            modhost_db::NewUser,
            modhost_db::NewUserToken,
            modhost_db::ProjectManifest,
            modhost_db::Project,
            modhost_db::ProjectAuthor,
            modhost_db::ProjectRelation,
            modhost_db::ProjectVersion,
            modhost_db::ProjectVersionRef,
            modhost_db::ProjectVersionInit,
            modhost_db::NewProject,
            modhost_db::NewProjectVersion,
            modhost_db::RelationKind,
            modhost_db::ProjectData,
            modhost_db::ProjectVisibility,
            modhost_db::GalleryImage,
            modhost_db::NewGalleryImage,
            modhost_db::PublicGalleryImage,
            modhost_db::ProjectFile,
            modhost_db::NewProjectFile,
            modhost_db::ProjectVersionData,
            modhost_search::Sort,
            modhost_search::SortMode,
            modhost_search::SearchResults,
            modhost_search::Facet,
            crate::api::JsonQueryParams,
            crate::pkg::info::PartialProject,
            crate::pkg::ver::PartialProjectVersion,
            crate::pkg::search::SearchQuery,
            crate::pkg::gallery::PartialGalleryImage,
            crate::pkg::gallery::GalleryImageUpload,
            modhost_server_core::models::GameVersion,
            modhost_server_core::models::ModLoader,
            modhost_server_core::models::Tag,
        ),
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints."),
        (name = "Users", description = "User-related endpoints."),
        (name = "Projects", description = "Project-related endpoints."),
        (name = "Gallery", description = "Project gallery-related endpoints."),
        (name = "Versions", description = "Project version-related endpoints."),
        (name = "Misc", description = "Miscellaneous endpoints."),
        (name = "Meta", description = "Metadata-related endpoints."),
    ),
    modifiers(
        &TokenAuthAddon,
    ),
)]
pub struct ApiDocs;

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
