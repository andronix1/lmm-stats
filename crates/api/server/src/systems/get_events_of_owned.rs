use common::{result::ApiResult, systems::get_events_of_owned::{GetEventsOfOwnedApiError, GetEventsOfOwnedApiEventInfo, GetEventsOfOwnedApiResult}};
use feature_events::domain::repo::EventsRepo;
use feature_systems::domain::repo::SystemsRepo;

pub struct GetEventsOfOwnedService;

impl GetEventsOfOwnedService {
    pub async fn get_events_of_system<T: EventsRepo + SystemsRepo>(&self, repo: &mut T, system: String) -> GetEventsOfOwnedApiResult {
        if !SystemsRepo::name_exists(repo, &system).await? {
            return ApiResult::Error(GetEventsOfOwnedApiError::NotFound);
        }
        ApiResult::Success(EventsRepo::get_of_system(repo, &system).await?.into_iter().map(|e| GetEventsOfOwnedApiEventInfo {
            name: e.name,
            human_name: e.human_name,
        }).collect())
    }
}