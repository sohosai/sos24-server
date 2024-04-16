use tokio::io::AsyncRead;

pub struct ArchiveToBeExportedDto<R: AsyncRead> {
    pub filename: String,
    pub body: R,
}
