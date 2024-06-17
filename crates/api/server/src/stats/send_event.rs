use common::{result::ApiResult, stats::send_event::{StatsSendEventApiError, StatsSendEventApiRequest, StatsSendEventApiResult}};
use feature_event_calls::domain::{models::CreateEventCallInfo, repo::EventCallsRepo};
use feature_events::domain::repo::EventsRepo;
use uuid::Uuid;

pub struct StatsSendEventService;

impl StatsSendEventService {
    pub async fn send_event<T: EventCallsRepo + EventsRepo>(&self, repo: &mut T, system: String, from_client: Uuid, request: StatsSendEventApiRequest) -> StatsSendEventApiResult {
        if !EventsRepo::exists(repo, &system, &request.name).await? {
            return ApiResult::Error(StatsSendEventApiError::InvalidEvent);
        }
        EventCallsRepo::add(repo, CreateEventCallInfo {
            system,
            from_client,
            event: request.name,
        }).await?;
        ApiResult::Success(())
    }
}