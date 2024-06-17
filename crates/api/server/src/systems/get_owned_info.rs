use common::{result::ApiResult, systems::{get_owned_info::{GetOwnedSystemInfoApiError, GetOwnedSystemInfoApiRequest, GetOwnedSystemInfoApiResult}, shared::systems::SystemInfoApiResponse}};
use feature_systems::domain::repo::SystemsRepo;

pub struct GetOwnedSystemInfoService;

impl GetOwnedSystemInfoService {
    pub async fn get_owned_info(&self, repo: &mut dyn SystemsRepo, request: GetOwnedSystemInfoApiRequest) -> GetOwnedSystemInfoApiResult {
        if !repo.name_exists(&request.name).await? {
            return ApiResult::Error(GetOwnedSystemInfoApiError::NotExists);
        }
        if let Some(info) = repo.try_get_owned_info(&request.name, request.owner).await? {
            ApiResult::Success(SystemInfoApiResponse {
                human_name: info.human_name,
                active: info.active,
                owner_login: info.owner_login,
                secret: info.secret,
            })
        } else {
            ApiResult::Forbidden
        }
    }
}