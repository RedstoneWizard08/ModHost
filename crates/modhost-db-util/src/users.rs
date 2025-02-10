//! Utilities for working with users.

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use itertools::Itertools;
use modhost_core::Result;
use modhost_db::{
    DbConn, Project, ProjectData, ProjectVisibility, User, project_authors, projects, users,
};

/// Get a list of projects for a user.
pub async fn get_user_projects(
    authed_user: Option<User>,
    user: i32,
    conn: &mut DbConn,
) -> Result<Vec<ProjectData>> {
    let mut query = projects::table
        .inner_join(project_authors::table.inner_join(users::table))
        .select((Project::as_select(), User::as_select()))
        .filter(users::id.eq(user))
        .into_boxed();

    if let Some(authed_user) = authed_user {
        if !authed_user.admin && !authed_user.moderator {
            query = query.filter(
                projects::visibility
                    .eq(ProjectVisibility::Public)
                    .or(users::id.eq(authed_user.id)),
            );
        }
    } else {
        query = query.filter(projects::visibility.eq(ProjectVisibility::Public));
    }

    Ok(
        (query.load::<(Project, User)>(conn).await? as Vec<(Project, User)>)
            .into_iter()
            .into_group_map()
            .into_iter()
            .map(|v| v.0.with_authors(v.1))
            .collect_vec(),
    )
}
