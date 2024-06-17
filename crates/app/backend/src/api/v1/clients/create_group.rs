use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{clients::create_group::CreateGroupService, common::clients::create_group::{CreateGroupApiError, CreateGroupApiRequest}};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn put(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<CreateGroupService>>,
    Json(request): Json<CreateGroupApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.create(tx, request), |err| match err {
        CreateGroupApiError::NameNotUnique => StatusCode::CONFLICT,
    })
}