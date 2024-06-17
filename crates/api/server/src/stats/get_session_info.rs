use std::sync::Arc;

use api_common::result::ApiResult;
use common::stats::get_session_info::{StatsGetSessionInfoApiResponse, StatsGetSessionInfoApiResult};
use feature_clients::domain::repo::ClientsRepo;
use feature_systems::domain::repo::SystemsRepo;
use crate::stats::shared::access_token::StatsAccessTokenClaims;

use super::shared::access_token::StatsAccessTokenGenerator;

pub struct StatsGetSessionInfoService {
    pub atg: Arc<dyn StatsAccessTokenGenerator>
}

impl StatsGetSessionInfoService {
    pub async fn get_session_info<T: SystemsRepo + ClientsRepo>(&self, repo: &mut T, access_token: String) -> StatsGetSessionInfoApiResult {
        let claims: StatsAccessTokenClaims = if let Ok(Ok(claims)) = self.atg.verify(access_token) {
            claims
        } else {
            return ApiResult::Unauthorized;
        };

        if !SystemsRepo::name_exists(repo, &claims.info.system).await? {
            return ApiResult::Unauthorized;
        }

        if let Some(client_id) = claims.info.client_id {
            if !ClientsRepo::client_exists(repo, client_id).await? {
                return ApiResult::Unauthorized;
            }
        }

        ApiResult::Success(StatsGetSessionInfoApiResponse {
            client_id: claims.info.client_id,
            system: claims.info.system,
        })
    }
}