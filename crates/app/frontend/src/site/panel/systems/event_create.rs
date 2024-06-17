use std::sync::Arc;

use api_common::systems::event_create::{SystemEventCreateApiRequest, SystemEventCreateApiResult};
use axum::{extract::Path, response::{IntoResponse, Redirect, Response}, Extension, Form};
use reqwest::Method;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::Session, tera::TeraTemplate};

const PANEL_SYSTEM_EVENTS_TEMPLATE: &'static str = "panel/systems/events/create.tera";

#[derive(Deserialize)]
pub struct GetEventsQuery {
    system: String,
}

pub async fn get(
    Extension(tera): Extension<Arc<Tera>>,
    Path(query): Path<GetEventsQuery>
) -> Response { 
    TeraTemplate::new(tera, PANEL_SYSTEM_EVENTS_TEMPLATE, {
        let mut ctx = Context::new();
        ctx.insert("systemName", &query.system);
        ctx
    }).into_response()
}

pub async fn post(
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>,
    mut session: Session,
    Path(query): Path<GetEventsQuery>,
    Form(request): Form<SystemEventCreateApiRequest>
) -> Response { 
    let result: SystemEventCreateApiResult = requester.request(Method::PUT, &format!("/api/v1/systems/my/{}/events", query.system), Some(&mut session), |builder| builder.json(&request)).await;
    ApiResultWrapper(result).respond(
        |_ok| {
            Redirect::to(&format!("/panel/systems/my/{}/events", query.system)).into_response()
        }, 
        |err| {
            TeraTemplate::new(tera, PANEL_SYSTEM_EVENTS_TEMPLATE, {
                let mut ctx = Context::new();
                ctx.insert("systemName", &query.system);
                ctx.insert("error", &err);
                ctx
            }).into_response()
        }
    )
}
