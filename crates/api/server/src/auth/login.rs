use std::sync::Arc;

use api_common::{auth::login::{LoginApiError, LoginApiRequest, LoginApiResult}, result::ApiResult};
use feature_users::domain::{Credentials, UsersRepo};

use crate::tokens_pair::{create_tokens_pair, RefreshTokenGenerator};

use super::shared::access_token::{AccessTokenGenerator, AccessTokenInfo};

pub struct LoginService {
    pub atg: Arc<dyn AccessTokenGenerator>,
    pub rtg: Arc<dyn RefreshTokenGenerator>,
}

impl LoginService {
    pub async fn login(&self, repo: &mut dyn UsersRepo, request: LoginApiRequest) -> LoginApiResult {
        let info = match UsersRepo::try_get_auth_info(repo, Credentials {
            login: request.login,
            password: request.password,
        }).await? {
            Some(info) => info,
            None => return ApiResult::Error(LoginApiError::InvalidCredentials),
        };
        ApiResult::Success(create_tokens_pair(AccessTokenInfo {
            id: info.id,
            user_revision: info.revision,
        }, self.atg.as_ref(), self.rtg.as_ref())?)
    }
}