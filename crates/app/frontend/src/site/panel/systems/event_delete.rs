use std::sync::Arc;

use api_common::systems::event_delete::SystemEventDeleteApiResult;
use axum::{extract::Path, response::{IntoResponse, Redirect, Response}, Extension};
use reqwest::Method;
use serde::Deserialize;

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, errors::internal::InternalError, session::Session};

#[derive(Deserialize)]
pub struct GetEventsQuery {
    system: String,
    event: String,
}

pub async fn post(
    Extension(requester): Extension<Arc<ApiRequester>>,
    mut session: Session,
    Path(query): Path<GetEventsQuery>
) -> Response { 
    let result: SystemEventDeleteApiResult = requester.request(Method::DELETE, &format!("/api/v1/systems/my/{}/events/{}", query.system, query.event), Some(&mut session), |builder| builder).await;
    ApiResultWrapper(result).respond(
        |_ok| {
            Redirect::to(&format!("/panel/systems/my/{}/events", query.system)).into_response()
        }, 
        |_err| {
            InternalError.into_response()
        }
    )
}