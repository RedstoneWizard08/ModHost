//! The S3 storage configuration.

use modhost_core::Result;
use object_store::aws::{AmazonS3, AmazonS3Builder};

/// The S3 storage configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// The S3 region to connect with.
    pub s3_region: String,

    /// The S3 endpoint to connect to.
    pub s3_endpoint: String,

    /// The S3 access key.
    pub s3_access_key: String,

    /// The S3 secret key.
    pub s3_secret_key: String,

    /// The bucket name for projects.
    /// Defaults to `"projects"`
    pub projects_bucket: String,

    /// The bucket name for project galleries.
    /// Defaults to `"gallery"`
    pub gallery_bucket: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            s3_region: String::new(),
            s3_endpoint: String::new(),
            s3_access_key: String::new(),
            s3_secret_key: String::new(),
            projects_bucket: "projects".into(),
            gallery_bucket: "gallery".into(),
        }
    }
}

impl StorageConfig {
    /// Get the [`AmazonS3`] bucket for projects.
    pub fn projects(&self) -> Result<AmazonS3> {
        Ok(AmazonS3Builder::new()
            .with_region(&self.s3_region)
            .with_endpoint(&self.s3_endpoint)
            .with_bucket_name(&self.projects_bucket)
            .with_access_key_id(&self.s3_access_key)
            .with_secret_access_key(&self.s3_secret_key)
            .build()?)
    }

    /// Get the [`AmazonS3`] bucket for project galleries.
    pub fn gallery(&self) -> Result<AmazonS3> {
        Ok(AmazonS3Builder::new()
            .with_region(&self.s3_region)
            .with_endpoint(&self.s3_endpoint)
            .with_bucket_name(&self.gallery_bucket)
            .with_access_key_id(&self.s3_access_key)
            .with_secret_access_key(&self.s3_secret_key)
            .build()?)
    }
}
