use std::sync::Arc;

use api_common::{auth::refresh_session::{RefreshSessionApiError, RefreshSessionApiRequest, RefreshSessionApiResult}, result::ApiResult};
use feature_users::domain::UsersRepo;

use crate::tokens_pair::{refresh_tokens_pair, RefreshTokenGenerator};

use super::shared::access_token::{AccessTokenGenerator, AccessTokenInfo};


pub struct RefreshSessionService {
    pub atg: Arc<dyn AccessTokenGenerator>,
    pub rtg: Arc<dyn RefreshTokenGenerator>,
}

impl RefreshSessionService {
    pub async fn refresh_session(&self, repo: &mut dyn UsersRepo, request: RefreshSessionApiRequest) -> RefreshSessionApiResult {
        match refresh_tokens_pair(async move |ati: &mut AccessTokenInfo| {
            Ok(repo.try_get_revision(ati.id).await? == Some(ati.user_revision))
        }, request, self.atg.as_ref(), self.rtg.as_ref()).await? {
            Ok(pair) => ApiResult::Success(pair),
            Err(_err) => ApiResult::Error(RefreshSessionApiError::InvalidTokens),
        }
    }
}