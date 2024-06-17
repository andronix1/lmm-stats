use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::systems::patch_owned::{PatchOwnedSystemApiError, PatchOwnedSystemApiRequest}, systems::patch_owned::PatchOwnedSystemService};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;

use crate::api::v1::auth::session::session::Session;

#[derive(Deserialize)]
pub struct PatchOwnedRequest {
    name: String
}

pub async fn patch(
    session: Session,
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<PatchOwnedSystemService>>,
    Path(path): Path<PatchOwnedRequest>,
    Json(request): Json<PatchOwnedSystemApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.patch_owned(tx, path.name, session.user_id, request), |err| match err {
        PatchOwnedSystemApiError::NotExists => StatusCode::NOT_FOUND,
        PatchOwnedSystemApiError::HumanNameNotUnique => StatusCode::CONFLICT, 
    })
}