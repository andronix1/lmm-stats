use common::{result::ApiResult, systems::get_events_of_owned::{GetEventsOfOwnedApiError, GetEventsOfOwnedApiEventInfo, GetEventsOfOwnedApiResult}};
use feature_events::domain::repo::EventsRepo;
use feature_systems::domain::repo::SystemsRepo;
use uuid::Uuid;

pub struct GetEventsOfOwnedService;

impl GetEventsOfOwnedService {
    pub async fn get_events_of_owned<T: EventsRepo + SystemsRepo>(&self, repo: &mut T, system: String, owner: Uuid) -> GetEventsOfOwnedApiResult {
        if !SystemsRepo::name_exists(repo, &system).await? {
            return ApiResult::Error(GetEventsOfOwnedApiError::NotFound);
        }
        if !SystemsRepo::has_owned(repo, &system, owner).await? {
            return ApiResult::Forbidden;
        }
        ApiResult::Success(EventsRepo::get_owned(repo, owner).await?.into_iter().map(|e| GetEventsOfOwnedApiEventInfo {
            name: e.name,
            human_name: e.human_name,
        }).collect())
    }
}