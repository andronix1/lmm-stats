use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::stats::get_clients_who::GetClientsWhoApiRequest, stats::get_clients_who::GetClientsWhoService};
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetClientsWhoSystem {
    system: String
}

pub async fn post(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<GetClientsWhoService>>,
    Path(path): Path<GetClientsWhoSystem>,
    Json(request): Json<GetClientsWhoApiRequest>
) -> impl IntoResponse { api_transactional!(db, |tx| service.get_clients_who(tx, path.system, request), |_err| unreachable!()) }