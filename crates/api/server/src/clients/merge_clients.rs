use common::{clients::merge_clients::{MergeClientsInGroupApiError, MergeClientsInGroupApiRequest, MergeClientsInGroupApiResult}, result::ApiResult};
use feature_clients::domain::repo::ClientsRepo;
use uuid::Uuid;

pub struct MergeClientsInGroupService;

impl MergeClientsInGroupService {
    pub async fn merge(&self, repo: &mut dyn ClientsRepo, group_id: Uuid, request: MergeClientsInGroupApiRequest) -> MergeClientsInGroupApiResult {
        if !repo.has_group_id(group_id).await? {
            return ApiResult::Error(MergeClientsInGroupApiError::GroupNotExists)
        }
        repo.merge_clients(group_id, request).await?;
        ApiResult::Success(())
    }
}