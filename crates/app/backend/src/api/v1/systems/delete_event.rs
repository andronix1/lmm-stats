use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::systems::event_delete::SystemEventDeleteApiError, systems::event_delete::SystemEventDeleteService};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;

use crate::api::v1::auth::session::session::Session;

#[derive(Deserialize)]
pub struct DeleteSystemRequest {
    system: String,
    event: String,
}

pub async fn delete(
    session: Session,
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<SystemEventDeleteService>>,
    Path(request): Path<DeleteSystemRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.delete_event(tx, session.user_id, request.system, request.event), |err| match err {
        SystemEventDeleteApiError::NotFound => StatusCode::NOT_FOUND,
    })
}