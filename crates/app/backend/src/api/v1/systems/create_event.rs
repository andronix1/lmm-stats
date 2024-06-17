use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::systems::event_create::{SystemEventCreateApiError, SystemEventCreateApiRequest}, systems::event_create::SystemEventCreateService};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;

use crate::api::v1::auth::session::session::Session;

#[derive(Deserialize)]
pub struct SystemEventCreateRequest {
    name: String
}

pub async fn put(
    session: Session,
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<SystemEventCreateService>>,
    Path(query): Path<SystemEventCreateRequest>,
    Json(request): Json<SystemEventCreateApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.create_event(tx, request, session.user_id, query.name), |err| match err {
        SystemEventCreateApiError::NameNotUnique => StatusCode::CONFLICT,
        SystemEventCreateApiError::HumanNameNotUnique => StatusCode::CONFLICT,
        SystemEventCreateApiError::NotFound => StatusCode::NOT_FOUND,
    })
}