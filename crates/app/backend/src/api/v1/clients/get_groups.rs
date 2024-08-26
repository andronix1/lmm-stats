use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::clients::get_groups::GetGroupsService;
use axum::{response::IntoResponse, Extension};

pub async fn get(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<GetGroupsService>>,
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.get_groups(tx), |_err: &()| unreachable!())
}