use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;

use api_server::{common::users::create::{CreateUserApiError, CreateUserApiRequest}, users::create_user::CreateUserService};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn put(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<CreateUserService>>,
    Json(request): Json<CreateUserApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.create(tx, request), |err| match err {
        CreateUserApiError::AlreasyExists => StatusCode::CONFLICT,
    })
}