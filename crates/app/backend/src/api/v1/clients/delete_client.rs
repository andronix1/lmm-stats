use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{clients::delete_client::DeleteClientService, common::clients::delete_client::DeleteClientApiError};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DeleteClientQuery {
    client_id: Uuid
}

pub async fn delete(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<DeleteClientService>>,
    Path(query): Path<DeleteClientQuery>,
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.delete(tx, query.client_id), |err| match err {
        DeleteClientApiError::NotExists => StatusCode::NOT_FOUND,
    })
}