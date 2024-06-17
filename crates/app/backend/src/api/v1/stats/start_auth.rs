use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::stats::auth::{StatsAuthApiError, StatsAuthApiRequest}, stats::auth::StatsAuthService};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use super::SystemScope;

pub async fn post(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<StatsAuthService>>,
    Path(scope): Path<SystemScope>,
    Json(request): Json<StatsAuthApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.auth(tx, scope.system, request), |err| match err {
        StatsAuthApiError::InvalidSystem => StatusCode::NOT_FOUND,
        StatsAuthApiError::InvalidAuthType | StatsAuthApiError::InvalidSecret => StatusCode::BAD_REQUEST,
    })
}