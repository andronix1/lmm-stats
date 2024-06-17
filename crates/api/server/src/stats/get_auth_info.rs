use api_common::result::ApiResult;
use common::stats::{get_auth_info::{StatsGetAuthInfoApiError, StatsGetAuthInfoApiRequest, StatsGetAuthInfoApiResponse, StatsGetAuthInfoApiResult}, StatsAuthType};
use feature_systems::domain::repo::SystemsRepo;

pub struct StatsGetAuthInfoService;

impl StatsGetAuthInfoService {
    pub async fn get_auth_info(&self, repo: &mut dyn SystemsRepo, request: StatsGetAuthInfoApiRequest) -> StatsGetAuthInfoApiResult {
        if let Some(active) = repo.try_get_active_status(&request.system).await? {
            if !active {
                return ApiResult::Error(StatsGetAuthInfoApiError::StatsNotActive);
            }
        } else {
            return ApiResult::Error(StatsGetAuthInfoApiError::InvalidSystem);
        }
        ApiResult::Success(StatsGetAuthInfoApiResponse {
            auth_type: if repo.has_secret(&request.system).await? { StatsAuthType::Secret } else { StatsAuthType::None },
        })
    }
}