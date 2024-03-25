use std::time::Duration;

use anyhow::Context;
use aws_sdk_s3::{presigning::PresigningConfig, primitives::SdkBody};
use sos24_domain::{
    entity::file_object::{ContentDisposition, FileObject, FileObjectKey, FileSignedUrl},
    repository::file_object::{FileObjectRepository, FileObjectRepositoryError},
};

use super::S3;

pub struct S3FileObjectRepository {
    s3: S3,
}

impl S3FileObjectRepository {
    pub fn new(s3: S3) -> Self {
        Self { s3 }
    }
}

impl FileObjectRepository for S3FileObjectRepository {
    async fn create(
        &self,
        bucket: String,
        object: FileObject,
    ) -> Result<(), FileObjectRepositoryError> {
        let raw_file_object = object.destruct();
        self.s3
            .put_object()
            .body(SdkBody::from(raw_file_object.data).into())
            .key(raw_file_object.key.value())
            .bucket(bucket)
            .send()
            .await
            .context("failed to create object")?;
        Ok(())
    }

    async fn generate_url(
        &self,
        bucket: String,
        key: FileObjectKey,
        content_disposition: Option<ContentDisposition>,
    ) -> Result<FileSignedUrl, FileObjectRepositoryError> {
        let presign_config = PresigningConfig::builder()
            .expires_in(Duration::new(3000, 0))
            .build()
            .context("Failed to build presigning config")?;
        let request = self
            .s3
            .get_object()
            .bucket(bucket)
            .key(key.value())
            .set_response_content_disposition(content_disposition.map(|value| value.value()))
            .presigned(presign_config)
            .await
            .context("Failed to generate presign url")?;
        // 間違ってそう
        Ok(FileSignedUrl::try_from(request.uri()).context("Failed to parse")?)
    }
}
