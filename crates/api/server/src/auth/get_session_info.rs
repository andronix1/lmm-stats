use std::sync::Arc;

use api_common::{auth::{self, get_session_info::{GetSessionInfoApiRequest, GetSessionInfoApiResponse, GetSessionInfoApiResult}}, result::ApiResult};
use feature_users::domain::{UserRole, UsersRepo};

use super::shared::access_token::AccessTokenGenerator;

pub struct GetSessionInfoService {
    pub atg: Arc<dyn AccessTokenGenerator>
}

impl GetSessionInfoService {
    pub async fn get_session_info(&self, repo: &mut dyn UsersRepo, request: GetSessionInfoApiRequest) -> GetSessionInfoApiResult {
        let claims = match self.atg.verify(request.access_token)? {
            Ok(claims) => claims,
            Err(_) => return ApiResult::Unauthorized
        };
        let info = match repo.try_get_access_info(claims.info.id).await? {
            Some(info) => info,
            None => return ApiResult::Unauthorized
        };
        if info.revision != claims.info.user_revision {
            return ApiResult::Unauthorized;
        }
        ApiResult::Success(GetSessionInfoApiResponse {
            role: match info.role {
                UserRole::Superuser => auth::UserRole::Superuser,
                UserRole::Developer => auth::UserRole::Developer,
                UserRole::Viewer => auth::UserRole::Viewer,
            },
            id: claims.info.id,
        })
    }
}