use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::systems::get_owned_info::{GetOwnedSystemInfoApiError, GetOwnedSystemInfoApiRequest}, systems::get_owned_info::GetOwnedSystemInfoService};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;

use crate::api::v1::auth::session::session::Session;

#[derive(Deserialize)]
pub struct GetOwnedSystemInfoRequest {
    name: String
}

pub async fn get(
    session: Session,
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<GetOwnedSystemInfoService>>,
    Path(request): Path<GetOwnedSystemInfoRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.get_owned_info(tx, GetOwnedSystemInfoApiRequest { 
        name: request.name, 
        owner: session.user_id
    }), |err| match err {
        GetOwnedSystemInfoApiError::NotExists => StatusCode::NOT_FOUND,
    })
}