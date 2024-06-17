use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::stats::finish_auth::StatsFinishAuthApiRequest, stats::finish_auth::StatsFinishAuthService};
use axum::{extract::Query, response::IntoResponse, Extension};

use super::session::session::StatsSession;

pub async fn post(
    session: StatsSession,
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<StatsFinishAuthService>>,
    Query(request): Query<StatsFinishAuthApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.finish_auth(tx, session.system, session.token, request), |_err| unreachable!())
}