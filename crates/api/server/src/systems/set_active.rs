use common::{result::ApiResult, systems::set_active::{SetSystemActiveApiResult, SetSystemActiveError, SetSystemActiveRequest}};
use feature_systems::domain::repo::SystemsRepo;

pub struct SetSystemActiveService;

impl SetSystemActiveService {
    pub async fn set_active(&self, repo: &mut dyn SystemsRepo, request: SetSystemActiveRequest) -> SetSystemActiveApiResult {
        if repo.try_set_active(&request.name, request.active).await? {
            if request.active == true {
                repo.mark_activated(&request.name).await?;
            }
            ApiResult::Success(())
        } else {
            ApiResult::Error(SetSystemActiveError::SystemNotExists)
        }
    }
}