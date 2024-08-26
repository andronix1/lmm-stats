use api_common::result::ApiResult;
use common::stats::get_clients_who::{GetClientsWhoApiRequest, GetClientsWhoApiResult};
use feature_event_calls::domain::repo::EventCallsRepo;

pub struct GetClientsWhoService;

impl GetClientsWhoService {
    pub async fn get_clients_who(&self, repo: &mut dyn EventCallsRepo, system: String, request: GetClientsWhoApiRequest) -> GetClientsWhoApiResult {
        ApiResult::Success(repo.get_clients_who(&system, request.group_id, request.events, request.after, request.before).await?)
    }
}