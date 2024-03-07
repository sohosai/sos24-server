use crate::entity::user::User;

pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: User) -> anyhow::Result<()>;
}
