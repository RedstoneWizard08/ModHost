//! Setup methods.

use crate::MeilisearchService;
use modhost_core::Result;

impl MeilisearchService {
    /// Ensure that filterable & sortable attributes are properly set up in the index.
    pub async fn ensure_setup(&self) -> Result<()> {
        if self
            .client
            .list_all_indexes()
            .await?
            .results
            .into_iter()
            .find(|v| v.uid == self.projects)
            .is_none()
        {
            self.client.create_index(&self.projects, None).await?;
        }

        // self.client.
        self.projects()
            .set_filterable_attributes(&[
                "id",
                "loaders",
                "game_versions",
                "name",
                "slug",
                "downloads",
                "source",
                "issues",
                "wiki",
                "license",
                "readme",
                "authors",
                "author_ids",
                "versions",
                "version_ids",
                "visibility",
                "created_at",
                "updated_at",
                "tags",
            ])
            .await?;

        self.projects()
            .set_sortable_attributes(&[
                "id",
                "loaders",
                "game_versions",
                "name",
                "slug",
                "downloads",
                "source",
                "issues",
                "wiki",
                "license",
                "readme",
                "authors",
                "author_ids",
                "versions",
                "version_ids",
                "visibility",
                "created_at",
                "updated_at",
                "tags",
            ])
            .await?;

        Ok(())
    }
}
