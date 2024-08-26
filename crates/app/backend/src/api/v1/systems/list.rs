use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::systems::list::GetSystemsListService;
use axum::{response::IntoResponse, Extension};

pub async fn get(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<GetSystemsListService>>,
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.list(tx), |_err: &()| unreachable!())
}