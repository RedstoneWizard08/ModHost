// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "moderation_status"))]
    pub struct ModerationStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "visibility"))]
    pub struct Visibility;
}

diesel::table! {
    gallery_images (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        ordering -> Int4,
        s3_id -> Text,
        project -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    moderation_comment (id) {
        id -> Int4,
        project_id -> Int4,
        user_id -> Int4,
        is_system -> Bool,
        is_moderator -> Bool,
        comment -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ModerationStatus;

    moderation_queue (id) {
        id -> Int4,
        project_id -> Int4,
        assigned_id -> Nullable<Int4>,
        status -> ModerationStatus,
    }
}

diesel::table! {
    project_authors (project, user_id) {
        project -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    project_relations (project, dependency, kind) {
        project -> Int4,
        dependency -> Int4,
        kind -> Int4,
    }
}

diesel::table! {
    project_version_refs (value) {
        value -> Int4,
    }
}

diesel::table! {
    project_versions (id) {
        id -> Int4,
        project -> Int4,
        name -> Text,
        version_number -> Text,
        changelog -> Nullable<Text>,
        loaders -> Array<Nullable<Text>>,
        game_versions -> Array<Nullable<Text>>,
        downloads -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Visibility;

    projects (id) {
        id -> Int4,
        name -> Text,
        slug -> Text,
        readme -> Text,
        description -> Text,
        source -> Nullable<Text>,
        issues -> Nullable<Text>,
        wiki -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        downloads -> Int4,
        license -> Nullable<Text>,
        visibility -> Visibility,
        tags -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    user_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        value -> Text,
        expires -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        github_id -> Int4,
        admin -> Bool,
        moderator -> Bool,
    }
}

diesel::table! {
    version_files (id) {
        id -> Int4,
        file_name -> Text,
        sha1 -> Text,
        s3_id -> Text,
        size -> Int8,
        version_id -> Int4,
        uploaded_at -> Timestamp,
    }
}

diesel::joinable!(gallery_images -> projects (project));
diesel::joinable!(moderation_comment -> projects (project_id));
diesel::joinable!(moderation_comment -> users (user_id));
diesel::joinable!(moderation_queue -> projects (project_id));
diesel::joinable!(moderation_queue -> users (assigned_id));
diesel::joinable!(project_authors -> projects (project));
diesel::joinable!(project_authors -> users (user_id));
diesel::joinable!(project_relations -> project_version_refs (dependency));
diesel::joinable!(project_relations -> project_versions (project));
diesel::joinable!(project_version_refs -> project_versions (value));
diesel::joinable!(project_versions -> projects (project));
diesel::joinable!(user_tokens -> users (user_id));
diesel::joinable!(version_files -> project_versions (version_id));

diesel::allow_tables_to_appear_in_same_query!(
    gallery_images,
    moderation_comment,
    moderation_queue,
    project_authors,
    project_relations,
    project_version_refs,
    project_versions,
    projects,
    user_tokens,
    users,
    version_files,
);
