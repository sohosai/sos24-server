use std::{ops::Deref, time::Duration};

use anyhow::Context;
use aws_sdk_s3::{presigning::PresigningConfig, primitives::SdkBody};
use sos24_domain::{
    entity::news_attachment_object::{
        NewsAttachmentObject, NewsAttachmentObjectKey, NewsAttachmentSignedUrl,
    },
    repository::news_attachment_object::{
        NewsAttachmentObjectRepository, NewsAttachmentObjectRepositoryError,
    },
};

use super::S3;

pub struct NewsAttachmentRepository {
    s3: S3,
}

impl NewsAttachmentRepository {
    pub fn new(s3: S3) -> Self {
        Self { s3 }
    }
}

impl NewsAttachmentObjectRepository for NewsAttachmentRepository {
    async fn create(
        &self,
        bucket: String,
        object: NewsAttachmentObject,
    ) -> Result<(), NewsAttachmentObjectRepositoryError> {
        self.s3
            .put_object()
            .body(SdkBody::from(object.data().deref()).into())
            .key(object.key().copy().value())
            .bucket(bucket)
            .send()
            .await
            .context("failed to create object")?;
        Ok(())
    }

    async fn generate_url(
        &self,
        bucket: String,
        key: NewsAttachmentObjectKey,
    ) -> Result<NewsAttachmentSignedUrl, NewsAttachmentObjectRepositoryError> {
        let presign_config = PresigningConfig::builder()
            .expires_in(Duration::new(3000, 0))
            .build()
            .context("Failed to build presigning config")?;
        let request = self
            .s3
            .get_object()
            .bucket(bucket)
            .key(key.value())
            .presigned(presign_config)
            .await
            .context("Failed to generate presign url")?;
        // 間違ってそう
        Ok(NewsAttachmentSignedUrl::try_from(request.uri()).context("Failed to parse")?)
    }
}
