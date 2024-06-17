use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::clients::get_clients::GetClientsService;
use axum::{response::IntoResponse, Extension};

use super::session::session::StatsSession;

pub async fn get(
    _session: StatsSession,
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<GetClientsService>>,
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.get(tx), |_err| unreachable!())
}