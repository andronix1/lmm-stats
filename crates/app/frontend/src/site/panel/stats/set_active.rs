use std::sync::Arc;

use api_common::systems::set_active::{SetSystemActiveApiResult, SetSystemActiveError};
use axum::{extract::{Path, Query}, response::{IntoResponse, Redirect, Response}, Extension};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, errors::internal::InternalError, session::Session};

use super::SystemPath;

#[derive(Deserialize, Serialize)]
pub struct SetSystemActiveQuery {
    active: bool
}

pub async fn get(
    Extension(requester): Extension<Arc<ApiRequester>>,
    mut session: Session,
    Path(path): Path<SystemPath>,
    Query(query): Query<SetSystemActiveQuery>
) -> Response { 
    let result: SetSystemActiveApiResult = requester.request(Method::PATCH, &format!("/api/v1/systems/{}/setActive", path.name), Some(&mut session), |builder| builder.query(&query)).await;
    ApiResultWrapper(result).respond(
        |_ok: ()| {
            Redirect::to("/panel/stats").into_response()
        }, |err| match err {
            SetSystemActiveError::SystemNotExists => InternalError.into_response(),
        }
    )
}