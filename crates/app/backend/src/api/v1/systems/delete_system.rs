use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::systems::delete::{DeleteSystemApiError, DeleteSystemApiRequest}, systems::delete::DeleteSystemService};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;

use crate::api::v1::auth::session::session::Session;

#[derive(Deserialize)]
pub struct DeleteSystemRequest {
    name: String
}

pub async fn delete(
    session: Session,
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<DeleteSystemService>>,
    Path(request): Path<DeleteSystemRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.delete(tx, DeleteSystemApiRequest { 
        name: request.name, 
        owner: session.user_id
    }), |err| match err {
        DeleteSystemApiError::NotExists => StatusCode::NOT_FOUND,
    })
}