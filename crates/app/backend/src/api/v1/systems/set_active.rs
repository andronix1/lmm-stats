use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::systems::set_active::{SetSystemActiveError, SetSystemActiveRequest}, systems::set_active::SetSystemActiveService};
use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SetSystemActivePath {
    name: String
}

#[derive(Deserialize)]
pub struct SetSystemActiveQuery {
    active: bool
}

pub async fn patch(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<SetSystemActiveService>>,
    Path(path): Path<SetSystemActivePath>,
    Query(query): Query<SetSystemActiveQuery>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.set_active(tx, SetSystemActiveRequest { name: path.name, active: query.active }), |err| match err {
        SetSystemActiveError::SystemNotExists => StatusCode::NOT_FOUND,
    })
}