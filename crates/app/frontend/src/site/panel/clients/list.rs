use std::sync::Arc;

use api_common::clients::get_clients::GetClientsApiResult;
use axum::{response::{IntoResponse, Response}, Extension};
use reqwest::Method;
use tera::{Context, Tera};

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::Session, tera::TeraTemplate};

const PANEL_CLIENTS_TEMPLATE: &'static str = "panel/clients/list.tera";

pub async fn get(
    mut session: Session,
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>
) -> Response {
    let result: GetClientsApiResult = requester.request(Method::GET, "/api/v1/clients", Some(&mut session), |builder| builder).await;
    ApiResultWrapper(result).respond(|ok| {
        TeraTemplate::new(tera, PANEL_CLIENTS_TEMPLATE, {
            let mut ctx = Context::new();
            ctx.insert("groups", &ok);
            ctx
        }).into_response()
    }, |_err| unreachable!())
}