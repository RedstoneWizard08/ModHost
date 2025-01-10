//! Utilities for indexing projects.

use crate::{MeiliProject, MeilisearchService};
use anyhow::anyhow;
use app_core::Result;
use db::{
    project_authors, project_versions, projects, users, DbConn, Project, ProjectVersion, User,
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use itertools::Itertools;
use meilisearch_sdk::documents::DocumentDeletionQuery;

impl MeilisearchService {
    /// Index all project present in the database.
    /// THIS IS A DESTRUCTIVE ACTION! IT WILL DELETE ALL EXISTING DATA
    /// IN THE MEILISEARCH INDEX!
    pub async fn index_projects(&self, conn: &mut DbConn) -> Result<()> {
        // This is my baby abomination and I am so proud of it.
        let projects: Vec<MeiliProject> = projects::table
            .inner_join(project_authors::table.inner_join(users::table))
            .inner_join(project_versions::table)
            .select((
                Project::as_select(),
                User::as_select(),
                ProjectVersion::as_select(),
            ))
            .load::<(Project, User, ProjectVersion)>(conn)
            .await?
            .into_iter()
            .into_group_map_by(|v: &(Project, User, ProjectVersion)| v.0.clone())
            .into_iter()
            .map(|v: (Project, Vec<(Project, User, ProjectVersion)>)| {
                (
                    v.0,
                    v.1.into_iter()
                        .map(|v| (v.1, v.2))
                        .unzip::<User, ProjectVersion, Vec<User>, Vec<ProjectVersion>>(),
                )
            })
            .map(|v| MeiliProject::from_data(v.0, v.1 .0, v.1 .1))
            .collect_vec();

        let index = self.projects();

        index.delete_all_documents().await?;
        index.add_documents(projects.as_slice(), Some("id")).await?;

        Ok(())
    }

    /// Update a project in the Meilisearch index.
    pub async fn update_project(&self, project: i32, conn: &mut DbConn) -> Result<()> {
        // Abomination #2! It's so beautiful! I make Rust programmers worldwide upset!
        let data: MeiliProject = projects::table
            .inner_join(project_authors::table.inner_join(users::table))
            .inner_join(project_versions::table)
            .select((
                Project::as_select(),
                User::as_select(),
                ProjectVersion::as_select(),
            ))
            .filter(projects::id.eq(project))
            .load::<(Project, User, ProjectVersion)>(conn)
            .await?
            .into_iter()
            .into_group_map_by(|v: &(Project, User, ProjectVersion)| v.0.clone())
            .into_iter()
            .map(|v: (Project, Vec<(Project, User, ProjectVersion)>)| {
                (
                    v.0,
                    v.1.into_iter()
                        .map(|v| (v.1, v.2))
                        .unzip::<User, ProjectVersion, Vec<User>, Vec<ProjectVersion>>(),
                )
            })
            .map(|v| MeiliProject::from_data(v.0, v.1 .0, v.1 .1))
            .find(|v| v.id == project)
            .ok_or(anyhow!("Could not find project with ID {}!", project))?;

        self.projects()
            .add_or_replace(&[data], Some("id"))
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        Ok(())
    }

    /// Delete a project from the Meilisearch index.
    pub async fn delete_project(&self, project: i32) -> Result<()> {
        let index = self.projects();
        let mut query = DocumentDeletionQuery::new(&index);
        let filter = format!("id = {}", project);

        query.with_filter(&filter);
        index.delete_documents_with(&query).await?;

        Ok(())
    }
}
