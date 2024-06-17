use std::sync::Arc;

use api_common::result::ApiResult;
use common::stats::finish_auth::{StatsFinishAuthApiRequest, StatsFinishAuthApiResponse, StatsFinishAuthApiResult};
use feature_systems::domain::repo::SystemsRepo;

use crate::tokens_pair::update_access_token;

use super::shared::access_token::{StatsAccessTokenGenerator, StatsAccessTokenInfo};

pub struct StatsFinishAuthService {
    pub atg: Arc<dyn StatsAccessTokenGenerator>,
}

impl StatsFinishAuthService {
    pub async fn finish_auth(&self, repo: &mut dyn SystemsRepo, system_name: String, access_token: String, request: StatsFinishAuthApiRequest) -> StatsFinishAuthApiResult {
        if !repo.name_exists(&system_name).await? {
            return ApiResult::Unauthorized;
        }
        
        match update_access_token(async |ati: &mut StatsAccessTokenInfo| {
            if ati.system != ati.system { 
                return Ok(false);
            }
            ati.client_id = Some(request.client_id);
            Ok(true)
        }, access_token, self.atg.as_ref()).await? {
            Ok(access_token) => ApiResult::Success(StatsFinishAuthApiResponse { access_token }),
            Err(_err) => ApiResult::Unauthorized,
        }
    }
}