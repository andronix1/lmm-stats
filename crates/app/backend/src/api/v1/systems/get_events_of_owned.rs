use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::systems::get_events_of_owned::GetEventsOfOwnedApiError, systems::get_events_of_owned::GetEventsOfOwnedService};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetEventsOfOwnedRequest {
    name: String
}

pub async fn get(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<GetEventsOfOwnedService>>,
    Path(query): Path<GetEventsOfOwnedRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.get_events_of_system(tx, query.name), |err| match err {
        GetEventsOfOwnedApiError::NotFound => StatusCode::NOT_FOUND,
    })
}