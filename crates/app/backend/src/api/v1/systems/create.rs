use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::systems::create::{CreateSystemApiError, CreateSystemApiRequest}, systems::create::CreateSystemService};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;

use crate::api::v1::auth::session::session::Session;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSystemRequest {
    name: String,
    human_name: String,
    secret: Option<String>,
}

pub async fn put(
    session: Session,
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<CreateSystemService>>,
    Json(request): Json<CreateSystemRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.create(tx, session.user_id, CreateSystemApiRequest { 
        name: request.name,
        human_name: request.human_name,
        secret: request.secret
    }), |err| match err {
        CreateSystemApiError::HumanNameNotUnique | CreateSystemApiError::NameNotUnique => StatusCode::CONFLICT,
    })
}