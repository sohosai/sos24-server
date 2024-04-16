use sos24_domain::entity::project::ProjectId;

use super::dto::ProjectWithOwnersDto;

#[allow(async_fn_in_trait)]
pub trait ProjectQueryService: Send + Sync + 'static {
    fn find_by_id(
        &self,
        id: ProjectId,
    ) -> impl std::future::Future<Output = anyhow::Result<ProjectWithOwnersDto>> + Send;
}
