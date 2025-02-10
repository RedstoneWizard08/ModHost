//! Utilities for working with gallery images.

use modhost_core::Result;
use modhost_db::{GalleryImage, PublicGalleryImage};
use object_store::{ObjectStore, aws::AmazonS3, path::Path};

/// Get a gallery image's bytes from S3.
pub async fn get_image(id: impl AsRef<str>, bucket: &AmazonS3) -> Result<Vec<u8>> {
    Ok(bucket
        .get(&Path::from(format!("/{}", id.as_ref())))
        .await?
        .bytes()
        .await?
        .to_vec())
}

/// Transform a [`GalleryImage`] into a [`PublicGalleryImage`], with the correct URL for it.
pub fn transform_gallery_image(img: GalleryImage) -> PublicGalleryImage {
    let url = format!("/api/v1/projects/s3/gallery/{}", img.s3_id);

    PublicGalleryImage {
        id: img.id,
        name: img.name,
        project: img.project,
        created_at: img.created_at,
        updated_at: img.updated_at,
        description: img.description,
        ordering: img.ordering,
        url,
    }
}

/// Transform an entire [`Vec`] of [`GalleryImage`]s into a [`Vec`] of [`PublicGalleryImage`]s.
pub fn transform_gallery(images: Vec<GalleryImage>) -> Vec<PublicGalleryImage> {
    let mut output = Vec::new();

    for img in images {
        output.push(transform_gallery_image(img));
    }

    output
}
