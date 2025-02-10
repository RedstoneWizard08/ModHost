//! The S3 storage configuration.

use modhost_core::Result;
use s3::{Bucket, Region, creds::Credentials};

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
    /// Get the S3 [`Credentials`] for this config.
    pub fn credentials(&self) -> Result<Credentials> {
        Ok(Credentials::new(
            Some(&self.s3_access_key),
            Some(&self.s3_secret_key),
            None,
            None,
            None,
        )?)
    }

    /// Get the S3 [`Region`] for this config.
    pub fn region(&self) -> Region {
        Region::Custom {
            region: self.s3_region.clone(),
            endpoint: self.s3_endpoint.clone(),
        }
    }

    /// Get the S3 [`Bucket`] for projects.
    pub fn projects(&self) -> Result<Box<Bucket>> {
        Ok(
            Bucket::new(&self.projects_bucket, self.region(), self.credentials()?)?
                .with_path_style(),
        )
    }

    /// Get the S3 [`Bucket`] for project galleries.
    pub fn gallery(&self) -> Result<Box<Bucket>> {
        Ok(
            Bucket::new(&self.gallery_bucket, self.region(), self.credentials()?)?
                .with_path_style(),
        )
    }
}
