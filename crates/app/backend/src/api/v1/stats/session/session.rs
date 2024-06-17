use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::stats::get_session_info::StatsGetSessionInfoApiResult, stats::get_session_info::StatsGetSessionInfoService};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, response::Response, Extension, RequestPartsExt};
use uuid::Uuid;

use crate::api::v1::response::{internal_error_arg, unauthorized_error};

#[derive(Clone)]
pub struct StatsSession {
    pub token: String,
    pub client_id: Option<Uuid>,
    pub system: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for StatsSession {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        log::trace!("extracting session...");
        if let Some(cache) = parts.extensions.get::<Self>() {
            log::trace!("using cached session info");
            return Ok(cache.clone());
        }

        let access_token = parts.headers.get("Authorization").and_then(|value| value.to_str().ok().and_then(|value| value.split_once(' ').and_then(|(token_type, token)| if token_type == "Bearer" {
            Some(token.to_owned()) } else { None }))).ok_or_else(unauthorized_error)?;

        let Extension(service) = parts.extract::<Extension<Arc<StatsGetSessionInfoService>>>().await.map_err(internal_error_arg)?;
        let Extension(db) = parts.extract::<Extension<Arc<SqlxDb>>>().await.map_err(internal_error_arg)?;
        let result = api_result_transactional!(db, |tx| service.get_session_info(tx, access_token.clone()));
        match result {
            StatsGetSessionInfoApiResult::Success(session) => Ok(StatsSession {
                token: access_token,
                client_id: session.client_id,
                system: session.system,
            }),
            StatsGetSessionInfoApiResult::Unauthorized => Err(unauthorized_error()),
            other => Err(internal_error_arg(other))
        }
    }
}
