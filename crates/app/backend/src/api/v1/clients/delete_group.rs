use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{clients::delete_group::DeleteGroupService, common::clients::delete_group::DeleteGroupApiError};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DeleteGroupQuery {
    group_id: Uuid
}

pub async fn delete(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<DeleteGroupService>>,
    Path(query): Path<DeleteGroupQuery>,
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.delete(tx, query.group_id), |err| match err {
        DeleteGroupApiError::NotExists => StatusCode::NOT_FOUND,
    })
}