use std::sync::Arc;

use api_common::result::ApiResult;
use common::stats::auth::{StatsAuthApiError, StatsAuthApiRequest, StatsAuthApiResult};
use feature_systems::domain::repo::SystemsRepo;

use crate::{stats::shared::access_token::{StatsAccessTokenGenerator, StatsAccessTokenInfo}, tokens_pair::{create_tokens_pair, RefreshTokenGenerator}};

pub struct StatsAuthService {
    pub atg: Arc<dyn StatsAccessTokenGenerator>,
    pub rtg: Arc<dyn RefreshTokenGenerator>
}

impl StatsAuthService {
    pub async fn auth(&self, repo: &mut dyn SystemsRepo, system_name: String, request: StatsAuthApiRequest) -> StatsAuthApiResult {
        if !repo.name_exists(&system_name).await? {
            return ApiResult::Error(StatsAuthApiError::InvalidSystem);
        }
        let system_secret = repo.get_secret(&system_name).await?;

        match request {
            StatsAuthApiRequest::Secret { secret } => {
                if system_secret.is_none() {
                    return ApiResult::Error(StatsAuthApiError::InvalidAuthType);
                }
                if system_secret != Some(secret) {
                    return ApiResult::Error(StatsAuthApiError::InvalidSecret);
                }
            },
            StatsAuthApiRequest::None => {
                if system_secret.is_some() {
                    return ApiResult::Error(StatsAuthApiError::InvalidAuthType);
                }
            },
        }

        ApiResult::Success(create_tokens_pair(StatsAccessTokenInfo { system: system_name, client_id: None }, self.atg.as_ref(), self.rtg.as_ref())?)
    }
}