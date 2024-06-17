use common::{result::ApiResult, systems::delete::{DeleteSystemApiError, DeleteSystemApiRequest, DeleteSystemApiResponse, DeleteSystemApiResult}};
use feature_systems::domain::repo::SystemsRepo;

pub struct DeleteSystemService;

impl DeleteSystemService {
    pub async fn delete(&self, repo: &mut dyn SystemsRepo, request: DeleteSystemApiRequest) -> DeleteSystemApiResult {
        if !repo.name_exists(&request.name).await? {
            return ApiResult::Error(DeleteSystemApiError::NotExists);
        }
        if !repo.try_delete_owned(&request.name, request.owner).await? {
            return ApiResult::Forbidden;
        }

        ApiResult::Success(DeleteSystemApiResponse)
    }
}