use common::{clients::delete_client::{DeleteClientApiError, DeleteClientApiResult}, result::ApiResult};
use feature_clients::domain::repo::ClientsRepo;
use uuid::Uuid;

pub struct DeleteClientService;

impl DeleteClientService {
    pub async fn delete(&self, repo: &mut dyn ClientsRepo, client_id: Uuid) -> DeleteClientApiResult {
        if !repo.try_delete_client(client_id).await? {
            ApiResult::Error(DeleteClientApiError::NotExists)
        } else {
            ApiResult::Success(())
        }
    }
}