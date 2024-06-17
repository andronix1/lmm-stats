use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{auth::refresh_session::RefreshSessionService, common::auth::refresh_session::{RefreshSessionApiError, RefreshSessionApiRequest}};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};


pub async fn post(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<RefreshSessionService>>,
    Json(request): Json<RefreshSessionApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.refresh_session(tx, request), |err| match err {
        RefreshSessionApiError::InvalidTokens => StatusCode::BAD_REQUEST,
    })
}