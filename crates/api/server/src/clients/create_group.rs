use common::{clients::create_group::{CreateGroupApiError, CreateGroupApiRequest, CreateGroupApiResult}, result::ApiResult};
use feature_clients::domain::repo::ClientsRepo;

pub struct CreateGroupService;

impl CreateGroupService {
    pub async fn create(&self, repo: &mut dyn ClientsRepo, request: CreateGroupApiRequest) -> CreateGroupApiResult {
        if repo.has_group_name(&request.name).await? {
            return ApiResult::Error(CreateGroupApiError::NameNotUnique);
        }
        repo.create_group(&request.name).await?;
        ApiResult::Success(())
    }
}