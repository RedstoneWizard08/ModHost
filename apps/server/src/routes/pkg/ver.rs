//! Routes concerning package versions.

use crate::{auth::get_user_from_req, state::AppState, util::versions::get_latest_version, Result};
use anyhow::anyhow;
use app_core::AppError;
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use db::{
    get_full_package, get_package, get_version, package_authors, package_versions, packages,
    NewPackageVersion, Package, PackageAuthor, PackageVersion, PackageVersionInit,
    PackageVisibility,
};
use diesel::{delete, insert_into, update, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use semver::Version;
use sha1::{Digest, Sha1};

/// Information for updaing a package version.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, ToResponse, Serialize, Deserialize,
)]
pub struct PartialPackageVersion {
    /// The display name of the version.
    #[serde(default)]
    pub name: Option<String>,

    /// The version number.
    /// This must be a string confirming to the [SemVer](https://semver.org/) standard.
    #[serde(default)]
    pub version_number: Option<String>,

    /// The version changelog.
    #[serde(default)]
    pub changelog: Option<String>,

    /// The mod loaders this version works on.
    #[serde(default)]
    pub loaders: Option<Vec<String>>,

    /// The game versions this version works on.
    #[serde(default)]
    pub game_versions: Option<Vec<String>>,
}

/// List Package Versions
///
/// List available versions for a specific package.
#[utoipa::path(
    get,
    path = "/api/v1/packages/{id}/versions",
    tag = "Versions",
    responses(
        (status = 200, description = "Found package versions!", body = Vec<PackageVersion>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package ID whose versions we are looking for."),
    ),
)]
#[debug_handler]
pub async fn list_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let pkg = get_full_package(id, &mut conn).await?;

    if pkg.visibility == PackageVisibility::Private {
        match get_user_from_req(&jar, &headers, &mut conn).await {
            Ok(user) => {
                if !pkg.authors.iter().any(|v| v.github_id == user.github_id) && !user.admin {
                    return Err(AppError::NotFound);
                }
            }

            Err(_) => return Err(AppError::NotFound),
        }
    }

    let versions = package_versions::table
        .filter(package_versions::package.eq(pkg.id))
        .select(PackageVersion::as_select())
        .load(&mut conn)
        .await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&versions)?))?)
}

/// Get Package Version
///
/// Get information about a specific package version
#[utoipa::path(
    get,
    path = "/api/v1/packages/{id}/versions/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Found package version!", body = PackageVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
)]
#[debug_handler]
pub async fn info_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((package, version)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let pkg = get_full_package(package, &mut conn).await?;

    if pkg.visibility == PackageVisibility::Private {
        match get_user_from_req(&jar, &headers, &mut conn).await {
            Ok(user) => {
                if !pkg.authors.iter().any(|v| v.github_id == user.github_id) && !user.admin {
                    return Err(AppError::NotFound);
                }
            }

            Err(_) => return Err(AppError::NotFound),
        }
    }

    let ver = get_version(pkg.id, version, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&ver)?))?)
}

/// Get Latest Package Version
///
/// Get information about the latest package version
#[utoipa::path(
    get,
    path = "/api/v1/packages/{id}/versions/latest",
    tag = "Versions",
    responses(
        (status = 200, description = "Found latest version!", body = PackageVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
    ),
)]
#[debug_handler]
pub async fn latest_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(package): Path<String>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let pkg = get_full_package(package, &mut conn).await?;

    if pkg.visibility == PackageVisibility::Private {
        match get_user_from_req(&jar, &headers, &mut conn).await {
            Ok(user) => {
                if !pkg.authors.iter().any(|v| v.github_id == user.github_id) && !user.admin {
                    return Err(AppError::NotFound);
                }
            }

            Err(_) => return Err(AppError::NotFound),
        }
    }

    let ver = get_latest_version(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&ver)?))?)
}

/// Download Package Version
///
/// Download a specific package version
#[utoipa::path(
    get,
    path = "/api/v1/packages/{id}/versions/{version}/download",
    tag = "Versions",
    responses(
        (status = 307, description = "Redirecting to download"),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
)]
#[debug_handler]
pub async fn download_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((package, version)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Vec<u8>> {
    let mut conn = state.pool.get().await?;
    let pkg = get_full_package(package, &mut conn).await?;

    if pkg.visibility == PackageVisibility::Private {
        match get_user_from_req(&jar, &headers, &mut conn).await {
            Ok(user) => {
                if !pkg.authors.iter().any(|v| v.github_id == user.github_id) && !user.admin {
                    return Err(AppError::NotFound);
                }
            }

            Err(_) => return Err(AppError::NotFound),
        }
    }

    let ver = get_version(pkg.id, version, &mut conn).await?;

    update(packages::table)
        .filter(packages::id.eq(pkg.id))
        .set((
            packages::downloads.eq(pkg.downloads + 1),
            packages::updated_at.eq(pkg.updated_at),
        ))
        .returning(Package::as_returning())
        .get_result(&mut conn)
        .await?;

    let ver = update(package_versions::table)
        .filter(package_versions::id.eq(ver.id))
        .set((
            package_versions::downloads.eq(ver.downloads + 1),
            package_versions::updated_at.eq(ver.updated_at),
        ))
        .returning(PackageVersion::as_returning())
        .get_result(&mut conn)
        .await?;

    state.search.update_package(pkg.id, &mut conn).await?;

    let bytes = state
        .buckets
        .packages
        .get_object(format!("/{}.tgz", ver.file_id))
        .await?
        .into_bytes()
        .to_vec();

    Ok(bytes)
}

/// Upload Package Version
///
/// Upload a new package version
#[utoipa::path(
    put,
    path = "/api/v1/packages/{id}/versions",
    tag = "Versions",
    responses(
        (status = 200, description = "Created package version!", body = PackageVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
    ),
    request_body(content = PackageVersionInit, description = "The version data"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn create_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
    mut data: Multipart,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_package(id, &mut conn).await?;

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(&mut conn)
        .await?;

    if authors.iter().find(|v| v.user_id == user.id).is_none() && !user.admin {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    let mut name = None;
    let mut version_number = None;
    let mut changelog = None;
    let mut loaders = None;
    let mut game_versions = None;
    let mut file = None;

    while let Ok(Some(field)) = data.next_field().await {
        match field
            .name()
            .ok_or(anyhow!("Could not find a name for a field!"))?
        {
            "name" => name = Some(field.text().await?),
            "version_number" => version_number = Some(field.text().await?),
            "changelog" => changelog = Some(field.text().await?),
            "loaders" => {
                loaders = Some(
                    field
                        .text()
                        .await?
                        .split(",")
                        .map(|v| Some(v.to_string()))
                        .collect::<Vec<_>>(),
                )
            }
            "game_versions" => {
                game_versions = Some(
                    field
                        .text()
                        .await?
                        .split(",")
                        .map(|v| Some(v.to_string()))
                        .collect::<Vec<_>>(),
                )
            }
            "file" => file = Some(field.bytes().await?),
            _ => {}
        }
    }

    if name.is_none() {
        Err(anyhow!("Missing field: 'name'"))?;
    }

    if version_number.is_none() {
        Err(anyhow!("Missing field: 'version_number'"))?;
    }

    if loaders.is_none() {
        Err(anyhow!("Missing field: 'loaders'"))?;
    }

    if game_versions.is_none() {
        Err(anyhow!("Missing field: 'game_versions'"))?;
    }

    if file.is_none() {
        Err(anyhow!("Missing field: 'file'"))?;
    }

    let name = name.unwrap();
    let version_number = version_number.unwrap();
    let loaders = loaders.unwrap();
    let game_versions = game_versions.unwrap();
    let file = file.unwrap();

    Version::parse(&version_number)?;

    if !(state.verifier)(file.clone()) {
        Err(anyhow!("Invalid package!"))?;
    }

    let mut hasher = Sha1::new();

    hasher.update(&file);

    let file_id = format!("{:x}", hasher.finalize());
    let file_name = format!("{}", file_id);

    state
        .buckets
        .packages
        .put_object(format!("/{}", file_name), &file)
        .await?;

    let data = NewPackageVersion {
        package: pkg.id,
        name,
        version_number,
        file_id,
        changelog,
        loaders,
        game_versions,
        downloads: 0,
    };

    update(packages::table)
        .filter(packages::id.eq(pkg.id))
        .set(packages::updated_at.eq(Utc::now().naive_utc()))
        .returning(Package::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    let ver = insert_into(package_versions::table)
        .values(&data)
        .returning(PackageVersion::as_returning())
        .get_result(&mut conn)
        .await?;

    state.search.update_package(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&ver)?))?)
}

/// Update Package Version
///
/// Update information about package version
#[utoipa::path(
    patch,
    path = "/api/v1/packages/{id}/versions/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Updated package version!", body = PackageVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
    request_body(content = PartialPackageVersion, description = "The information to update"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn update_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((package, version)): Path<(String, String)>,
    State(state): State<AppState>,
    Json(data): Json<PartialPackageVersion>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_package(package, &mut conn).await?;
    let ver = get_version(pkg.id, version, &mut conn).await?;

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(&mut conn)
        .await?;

    if authors.iter().find(|v| v.user_id == user.id).is_none() && !user.admin {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    if let Some(ver_num) = &data.version_number {
        Version::parse(ver_num)?;
    }

    let ver = update(package_versions::table)
        .filter(package_versions::id.eq(ver.id))
        .set((
            package_versions::name.eq(data.name.unwrap_or(ver.name)),
            package_versions::version_number.eq(data.version_number.unwrap_or(ver.version_number)),
            package_versions::changelog
                .eq(data.changelog.map(|v| Some(v)).unwrap_or(ver.changelog)),
            package_versions::loaders.eq(data
                .loaders
                .map(|v| v.iter().map(|v| Some(v.clone())).collect::<Vec<_>>())
                .unwrap_or(ver.loaders)),
            package_versions::game_versions.eq(data
                .game_versions
                .map(|v| v.iter().map(|v| Some(v.clone())).collect::<Vec<_>>())
                .unwrap_or(ver.game_versions)),
            package_versions::updated_at.eq(Utc::now().naive_utc()),
        ))
        .returning(PackageVersion::as_select())
        .get_result(&mut conn)
        .await?;

    state.search.update_package(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&ver)?))?)
}

/// Delete Package Version
///
/// Delete a package version
#[utoipa::path(
    delete,
    path = "/api/v1/packages/{id}/versions/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Deleted package version!", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn delete_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((package, version)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_package(package, &mut conn).await?;
    let ver = get_version(pkg.id, version, &mut conn).await?;

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(&mut conn)
        .await?;

    if authors.iter().find(|v| v.user_id == user.id).is_none() && !user.admin {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    let all_referencing = package_versions::table
        .filter(package_versions::file_id.eq(ver.file_id.clone()))
        .select(PackageVersion::as_select())
        .load(&mut conn)
        .await?;

    if all_referencing.len() <= 1 {
        state
            .buckets
            .packages
            .delete_object(format!("/{}", ver.file_id))
            .await?;
    }

    delete(package_versions::table)
        .filter(package_versions::id.eq(ver.id))
        .execute(&mut conn)
        .await?;

    state.search.update_package(pkg.id, &mut conn).await?;

    Ok(Response::builder().body(Body::new(
        "Deleted package version successfully!".to_string(),
    ))?)
}
