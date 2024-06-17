use std::sync::Arc;

use api_common::systems::get_events_of_owned::GetEventsOfOwnedApiResult;
use axum::{extract::Path, response::{IntoResponse, Response}, Extension};
use reqwest::Method;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::Session, tera::TeraTemplate};

const PANEL_SYSTEM_EVENTS_TEMPLATE: &'static str = "panel/systems/events.tera";

#[derive(Deserialize)]
pub struct GetEventsQuery {
    name: String
}

async fn render_page(
    tera: Arc<Tera>,
    requester: Arc<ApiRequester>,
    mut session: Session,
    mut ctx: Context,
    name: String
) -> Response { 
    let result: GetEventsOfOwnedApiResult = requester.request(Method::GET, &format!("/api/v1/systems/my/{}/events", name), Some(&mut session), |builder| builder).await;
    ApiResultWrapper(result).respond(
        |ok| {
            TeraTemplate::new(tera, PANEL_SYSTEM_EVENTS_TEMPLATE, {
                ctx.insert("systemName", &name);
                ctx.insert("events", &ok);
                ctx
            }).into_response()
        }, 
        |_| unreachable!()
    )
}

pub async fn get(
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>,
    session: Session,
    Path(query): Path<GetEventsQuery>
) -> Response { render_page(tera, requester, session, Context::new(), query.name).await }
