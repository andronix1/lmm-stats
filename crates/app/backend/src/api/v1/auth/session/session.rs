use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{auth::get_session_info::GetSessionInfoService, common::auth::get_session_info::{GetSessionInfoApiRequest, GetSessionInfoApiResult}};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, response::Response, Extension, RequestPartsExt};
use feature_users::domain::UserRole;
use uuid::Uuid;

use crate::api::v1::response::{internal_error_arg, unauthorized_error};

#[derive(Clone)]
pub struct Session {
    pub user_id: Uuid,
    pub role: UserRole,
}

#[async_trait]
impl<S> FromRequestParts<S> for Session {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        log::trace!("extracting session...");
        if let Some(cache) = parts.extensions.get::<Self>() {
            log::trace!("using cached session info");
            return Ok(cache.clone());
        }

        let access_token = parts.headers.get("Authorization").and_then(|value| value.to_str().ok().and_then(|value| value.split_once(' ').and_then(|(token_type, token)| if token_type == "Bearer" {
            Some(token.to_owned()) } else { None }))).ok_or_else(unauthorized_error)?;

        let Extension(service) = parts.extract::<Extension<Arc<GetSessionInfoService>>>().await.map_err(internal_error_arg)?;
        let Extension(db) = parts.extract::<Extension<Arc<SqlxDb>>>().await.map_err(internal_error_arg)?;
        let result = api_result_transactional!(db, |tx| service.get_session_info(tx, GetSessionInfoApiRequest { access_token }));
        match result {
            GetSessionInfoApiResult::Success(session) => Ok(Session {
                user_id: session.id,
                role: session.role.into()
            }),
            GetSessionInfoApiResult::Unauthorized => Err(unauthorized_error()),
            other => Err(internal_error_arg(other))
        }
    }
}
