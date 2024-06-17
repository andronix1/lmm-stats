use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::stats::get_auth_info::{StatsGetAuthInfoApiError, StatsGetAuthInfoApiRequest}, stats::get_auth_info::StatsGetAuthInfoService};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};

pub async fn get(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<StatsGetAuthInfoService>>,
    Path(request): Path<StatsGetAuthInfoApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.get_auth_info(tx, request), |err| match err {
        StatsGetAuthInfoApiError::InvalidSystem => StatusCode::NOT_FOUND,
        StatsGetAuthInfoApiError::StatsNotActive => StatusCode::FORBIDDEN,
    })
}