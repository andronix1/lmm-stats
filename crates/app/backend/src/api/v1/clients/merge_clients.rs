use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{clients::merge_clients::MergeClientsInGroupService, common::clients::merge_clients::MergeClientsInGroupApiRequest};
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct MergeClientsQuery {
    group_id: Uuid
}

pub async fn patch(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<MergeClientsInGroupService>>,
    Path(query): Path<MergeClientsQuery>,
    Json(request): Json<MergeClientsInGroupApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.merge(tx, query.group_id, request), |_err| unreachable!())
}