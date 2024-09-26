use std::path::PathBuf;
use std::time::Duration;

use anyhow::Context;
use async_zip::tokio::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder};
use aws_sdk_s3::{presigning::PresigningConfig, primitives::SdkBody};
use futures_util::future;
use tokio::io::DuplexStream;
use tokio_util::compat::FuturesAsyncWriteCompatExt;

use sos24_domain::entity::file_object::ArchiveEntry;
use sos24_domain::{
    entity::file_object::{ContentDisposition, FileObject, FileObjectKey, FileSignedUrl},
    repository::file_object::{FileObjectRepository, FileObjectRepositoryError},
};

use crate::shared::s3::S3;

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
        entry_list: Vec<ArchiveEntry>,
        writer: DuplexStream,
    ) -> Result<(), FileObjectRepositoryError> {
        tracing::info!("ファイルのアーカイブを作成します");

        let temp_dir = tempfile::tempdir().context("Failed to create temp dir")?;

        let mut tasks = Vec::new();
        for entry in entry_list {
            let s3 = self.s3.clone();
            let bucket = bucket.clone();
            let temp_dir_path = temp_dir.path().to_owned();

            let task = tokio::spawn(async move {
                let entry = entry.destruct();
                let entry_key = entry.key.value();

                tracing::info!("ファイルをダウンロードします: {:?}", entry_key);

                let file_data = s3
                    .get_object()
                    .bucket(&bucket)
                    .key(entry_key.clone())
                    .send()
                    .await
                    .context("Failed to get object")?;
                let mut file_data_stream = file_data.body.into_async_read();

                let temp_file_name = entry.filename.value();
                let temp_file_path = temp_dir_path.join(&temp_file_name);
                let mut temp_file = tokio::fs::File::create_new(&temp_file_path)
                    .await
                    .context("Failed to open file")?;

                tokio::io::copy_buf(&mut file_data_stream, &mut temp_file)
                    .await
                    .context("Failed to copy")?;

                tracing::info!("ファイルをダウンロードしました: {:?}", entry_key);

                Ok::<_, anyhow::Error>((temp_file_path, temp_file_name, entry.updated_at.value()))
            });
            tasks.push(task);
        }

        let temp_file_paths: Vec<(PathBuf, String, chrono::DateTime<chrono::Utc>)> =
            future::try_join_all(tasks)
                .await
                .context("Failed to join")?
                .into_iter()
                .collect::<Result<_, _>>()?;

        let mut zip_writer = ZipFileWriter::with_tokio(writer);
        for (temp_file_path, file_name, updated_at) in temp_file_paths {
            tracing::info!("ファイルをアーカイブに追加します: {:?}", temp_file_path);

            let mut temp_file = tokio::fs::File::open(temp_file_path.clone())
                .await
                .context("Failed to open file")?;

            let zip_entry = ZipEntryBuilder::new(file_name.into(), Compression::Deflate)
                .last_modification_date(updated_at.into());
            let mut zip_entry_stream = zip_writer
                .write_entry_stream(zip_entry)
                .await
                .context("Failed to write entry")?
                .compat_write();

            tokio::io::copy(&mut temp_file, &mut zip_entry_stream)
                .await
                .context("Failed to copy")?;

            zip_entry_stream
                .into_inner()
                .close()
                .await
                .context("Failed to close")?;

            tracing::info!("ファイルをアーカイブに追加しました: {:?}", temp_file_path);
        }

        zip_writer.close().await.context("Failed to close")?;

        tracing::info!("ファイルのアーカイブを作成しました");
        Ok(())
    }
}
