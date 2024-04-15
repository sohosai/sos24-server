use std::time::Duration;

use anyhow::Context;
use async_zip::tokio::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder};
use aws_sdk_s3::{presigning::PresigningConfig, primitives::SdkBody};
use tokio::io::DuplexStream;
use tokio_util::compat::FuturesAsyncWriteCompatExt;

use sos24_domain::entity::common::date::WithDate;
use sos24_domain::entity::file_data::FileData;
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
        tracing::info!("ファイルをS3にアップロードします");

        let raw_file_object = object.destruct();
        self.s3
            .put_object()
            .body(SdkBody::from(raw_file_object.data).into())
            .key(raw_file_object.key.value())
            .bucket(bucket)
            .send()
            .await
            .context("failed to create object")?;

        tracing::info!("ファイルのアップロードが完了しました");
        Ok(())
    }

    async fn generate_url(
        &self,
        bucket: String,
        key: FileObjectKey,
        content_disposition: Option<ContentDisposition>,
    ) -> Result<FileSignedUrl, FileObjectRepositoryError> {
        tracing::info!("ファイルの署名付きURLを生成します: {key:?}");

        let presign_config = PresigningConfig::builder()
            .expires_in(Duration::new(3000, 0))
            .build()
            .context("Failed to build presigning config")?;
        let request = self
            .s3
            .get_object()
            .bucket(bucket)
            .key(key.clone().value())
            .set_response_content_disposition(content_disposition.map(|value| value.value()))
            .presigned(presign_config)
            .await
            .context("Failed to generate presign url")?;

        tracing::info!("ファイルの署名付きURLを生成しました: {key:?}");
        Ok(FileSignedUrl::try_from(request.uri()).context("Failed to parse")?)
    }

    async fn create_archive(
        &self,
        bucket: String,
        files: Vec<WithDate<FileData>>,
    ) -> Result<DuplexStream, FileObjectRepositoryError> {
        tracing::info!("ファイルのアーカイブを作成します");

        let (writer, reader) = tokio::io::duplex(65535);
        let mut zip_writer = ZipFileWriter::with_tokio(writer);

        for file in files {
            let file_key = file.value.url().clone().value();
            let file_data = self
                .s3
                .get_object()
                .bucket(&bucket)
                .key(file_key)
                .send()
                .await
                .context("Failed to get object")?;
            let mut file_data_stream = file_data.body.into_async_read();

            let file_name = file.value.filename().clone().value();
            let zip_entry = ZipEntryBuilder::new(file_name.into(), Compression::Deflate)
                .last_modification_date(file.updated_at.into());
            let mut zip_entry_stream = zip_writer
                .write_entry_stream(zip_entry)
                .await
                .context("Failed to write entry")?
                .compat_write();

            tokio::io::copy(&mut file_data_stream, &mut zip_entry_stream)
                .await
                .context("Failed to copy")?;

            zip_entry_stream
                .into_inner()
                .close()
                .await
                .context("Failed to close")?;
        }

        zip_writer.close().await.context("Failed to close")?;

        tracing::info!("ファイルのアーカイブを作成しました");
        Ok(reader)
    }
}
