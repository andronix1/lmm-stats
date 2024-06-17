use std::sync::Arc;

use api_common::systems::get_owned::GetOwnedSystemsApiResult;
use axum::{response::{IntoResponse, Response}, Extension};
use reqwest::Method;
use tera::{Context, Tera};

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::Session, tera::TeraTemplate};

const PANEL_SYSTEMS_TEMPLATE: &'static str = "panel/systems/owned.tera";

pub async fn get(
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>,
    mut session: Session
) -> Response {
    let result: GetOwnedSystemsApiResult = requester.request(Method::GET, "/api/v1/systems/my", Some(&mut session), |builder| builder).await;
    ApiResultWrapper(result).respond(
        |ok| {
            TeraTemplate::new(tera, PANEL_SYSTEMS_TEMPLATE, {
                let mut ctx = Context::new();
                ctx.insert("systems", &ok);
                ctx
            }).into_response()
        }, 
        |_| unreachable!()
    )
}