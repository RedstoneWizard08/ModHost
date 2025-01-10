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
        crate::routes::api::yaml_api,
        crate::routes::api::json_api,
        crate::routes::users::me::me_handler,
        crate::routes::users::info::info_handler,
        crate::routes::users::pkg::list_handler,
        crate::routes::auth::login::login_handler,
        crate::routes::auth::callback::callback_handler,
        crate::routes::pkg::info::project_info_handler,
        crate::routes::pkg::info::update_project_handler,
        crate::routes::pkg::info::delete_project_handler,
        crate::routes::pkg::info::create_project_handler,
        crate::routes::pkg::ver::list_versions_handler,
        crate::routes::pkg::ver::version_info_handler,
        crate::routes::pkg::ver::download_version_handler,
        crate::routes::pkg::ver::create_version_handler,
        crate::routes::pkg::ver::update_version_handler,
        crate::routes::pkg::ver::delete_version_handler,
        crate::routes::pkg::ver::latest_version_handler,
        crate::routes::pkg::author::list_authors_handler,
        crate::routes::pkg::author::add_author_handler,
        crate::routes::pkg::author::remove_author_handler,
        crate::routes::pkg::search::search_projects_handler,
        crate::routes::pkg::gallery::list_gallery_handler,
        crate::routes::pkg::gallery::upload_gallery_handler,
        crate::routes::pkg::gallery::update_gallery_handler,
        crate::routes::pkg::gallery::delete_gallery_handler,
        crate::routes::pkg::gallery::s3_image_handler,
        crate::routes::pkg::gallery::gallery_info_handler,
        crate::routes::meta::badge::version_handler,
        crate::routes::meta::badge::latest_version_badge_handler,
        crate::routes::meta::vers::game_versions_handler,
        crate::routes::meta::loaders::loaders_handler,
        crate::routes::meta::tags::tags_handler,
    ),
    components(
        schemas(
            db::User,
            db::UserToken,
            db::NewUser,
            db::NewUserToken,
            db::ProjectManifest,
            db::Project,
            db::ProjectAuthor,
            db::ProjectRelation,
            db::ProjectVersion,
            db::ProjectVersionRef,
            db::ProjectVersionInit,
            db::NewProject,
            db::NewProjectVersion,
            db::RelationKind,
            db::ProjectData,
            db::ProjectVisibility,
            db::GalleryImage,
            db::NewGalleryImage,
            db::PublicGalleryImage,
            db::ProjectFile,
            db::NewProjectFile,
            db::ProjectVersionData,
            search::Sort,
            search::SortMode,
            search::SearchResults,
            search::Facet,
            crate::routes::api::JsonQueryParams,
            crate::routes::pkg::info::PartialProject,
            crate::routes::pkg::ver::PartialProjectVersion,
            crate::routes::pkg::search::SearchQuery,
            crate::routes::pkg::gallery::PartialGalleryImage,
            crate::routes::pkg::gallery::GalleryImageUpload,
            crate::routes::meta::vers::GameVersion,
            crate::routes::meta::loaders::ModLoader,
            crate::routes::meta::tags::Tag,
        ),
        responses(
            db::User,
            db::UserToken,
            db::NewUser,
            db::NewUserToken,
            db::ProjectManifest,
            db::Project,
            db::ProjectAuthor,
            db::ProjectRelation,
            db::ProjectVersion,
            db::ProjectVersionRef,
            db::ProjectVersionInit,
            db::NewProject,
            db::NewProjectVersion,
            db::RelationKind,
            db::ProjectData,
            db::ProjectVisibility,
            db::GalleryImage,
            db::NewGalleryImage,
            db::PublicGalleryImage,
            db::ProjectFile,
            db::NewProjectFile,
            db::ProjectVersionData,
            search::Sort,
            search::SortMode,
            search::SearchResults,
            search::Facet,
            crate::routes::api::JsonQueryParams,
            crate::routes::pkg::info::PartialProject,
            crate::routes::pkg::ver::PartialProjectVersion,
            crate::routes::pkg::search::SearchQuery,
            crate::routes::pkg::gallery::PartialGalleryImage,
            crate::routes::pkg::gallery::GalleryImageUpload,
            crate::routes::meta::vers::GameVersion,
            crate::routes::meta::loaders::ModLoader,
            crate::routes::meta::tags::Tag,
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
