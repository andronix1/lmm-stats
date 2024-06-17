use common::{clients::delete_group::{DeleteGroupApiError, DeleteGroupApiResult}, result::ApiResult};
use feature_clients::domain::repo::ClientsRepo;
use uuid::Uuid;

pub struct DeleteGroupService;

impl DeleteGroupService {
    pub async fn delete(&self, repo: &mut dyn ClientsRepo, group_id: Uuid) -> DeleteGroupApiResult {
        if !repo.try_delete_group(group_id).await? {
            ApiResult::Error(DeleteGroupApiError::NotExists)
        } else {
            ApiResult::Success(())
        }
    }
}