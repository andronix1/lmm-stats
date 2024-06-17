use common::{result::ApiResult, systems::event_create::{SystemEventCreateApiError, SystemEventCreateApiRequest, SystemEventCreateApiResult}};
use feature_events::domain::{models::CreateEventInfo, repo::EventsRepo};
use feature_systems::domain::repo::SystemsRepo;
use uuid::Uuid;

pub struct SystemEventCreateService;

impl SystemEventCreateService {
    pub async fn create_event<T: SystemsRepo + EventsRepo>(&self, repo: &mut T, info: SystemEventCreateApiRequest, owner: Uuid, system: String) -> SystemEventCreateApiResult {
        if !SystemsRepo::has_owned(repo, &system, owner).await? {
            if SystemsRepo::name_exists(repo, &system).await? {
                return ApiResult::Forbidden;
            }
            return ApiResult::Error(SystemEventCreateApiError::NotFound);
        }
        if EventsRepo::exists(repo, &system, &info.name).await? {
            return ApiResult::Error(SystemEventCreateApiError::NameNotUnique);
        }
        if EventsRepo::system_human_name_exists(repo, &system, &info.human_name).await? {
            return ApiResult::Error(SystemEventCreateApiError::HumanNameNotUnique);
        }
        ApiResult::Success(EventsRepo::create(repo, CreateEventInfo {
            name: info.name,
            system,
            human_name: info.human_name,
        }).await?)
    }
}