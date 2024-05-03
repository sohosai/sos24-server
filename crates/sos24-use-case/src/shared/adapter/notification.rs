use mockall::automock;

#[automock]
#[allow(async_fn_in_trait)]
pub trait Notifier: Send + Sync + 'static {
    async fn notify(&self, message: String) -> anyhow::Result<()>;
}
