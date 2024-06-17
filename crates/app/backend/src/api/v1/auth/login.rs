use std::sync::Arc;

use abstract_db::transactional;
use abstract_sqlx_bindings::SqlxDb;
use api_server::{auth::login::LoginService, common::auth::login::{LoginApiError, LoginApiRequest}};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn post(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<LoginService>>,
    Json(request): Json<LoginApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.login(tx, request), |err| match err {
        LoginApiError::InvalidCredentials => StatusCode::BAD_REQUEST,
    })
}