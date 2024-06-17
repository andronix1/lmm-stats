use std::sync::Arc;

use api_common::result::ApiResult;
use common::stats::refresh_session::{StatsRefreshSessionApiRequest, StatsRefreshSessionApiResult};
use feature_clients::domain::repo::ClientsRepo;
use feature_systems::domain::repo::SystemsRepo;

use crate::{stats::shared::access_token::StatsAccessTokenGenerator, tokens_pair::{refresh_tokens_pair, RefreshTokenGenerator}};

use super::shared::access_token::StatsAccessTokenInfo;

pub struct StatsRefreshSessionService {
    pub atg: Arc<dyn StatsAccessTokenGenerator>,
    pub rtg: Arc<dyn RefreshTokenGenerator>
}

impl StatsRefreshSessionService {
    pub async fn refresh_session<T: SystemsRepo + ClientsRepo>(&self, repo: &mut T, request: StatsRefreshSessionApiRequest) -> StatsRefreshSessionApiResult {
        match refresh_tokens_pair(async |ati: &mut StatsAccessTokenInfo| {
            if !SystemsRepo::name_exists(repo, &ati.system).await? {
                return Ok(false);
            }
            if let Some(user_id) = ati.client_id {
                if !ClientsRepo::client_exists(repo, user_id).await? {
                    return Ok(false);
                }
            }
            return Ok(true);
        }, request, self.atg.as_ref(), self.rtg.as_ref()).await {
            Ok(Ok(pair)) => ApiResult::Success(pair),
            _ => ApiResult::Unauthorized,
        }
    }
}