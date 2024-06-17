use common::{result::ApiResult, systems::event_delete::{SystemEventDeleteApiError, SystemEventDeleteApiResult}};
use feature_events::domain::repo::EventsRepo;
use feature_systems::domain::repo::SystemsRepo;
use uuid::Uuid;

pub struct SystemEventDeleteService;

impl SystemEventDeleteService {
    pub async fn delete_event<T: SystemsRepo + EventsRepo>(&self, repo: &mut T, owner: Uuid, system: String, name: String) -> SystemEventDeleteApiResult {
        if !SystemsRepo::has_owned(repo, &system, owner).await? {
            if SystemsRepo::name_exists(repo, &system).await? {
                return ApiResult::Forbidden;
            }
            return ApiResult::Error(SystemEventDeleteApiError::NotFound);
        }
        if !EventsRepo::try_delete_owned(repo, &system, &name, owner).await? {
            return ApiResult::Error(SystemEventDeleteApiError::NotFound);
        }
        ApiResult::Success(())
    }
}