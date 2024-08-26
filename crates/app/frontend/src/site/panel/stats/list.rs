use std::sync::Arc;

use api_common::{clients::get_groups::GetGroupsApiResult, systems::list::GetSystemsListApiResult};
use axum::{response::{IntoResponse, Response}, Extension};
use reqwest::Method;
use tera::{Context, Tera};

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, errors::internal::InternalError, session::Session, tera::TeraTemplate};

const PANEL_STATS_LIST_TEMPLATE: &'static str = "panel/stats/list.tera";

pub async fn get(
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>,
    mut session: Session,
) -> Response { 
    let systems_response: GetSystemsListApiResult = requester.request(Method::GET, "/api/v1/systems/list", Some(&mut session), |builder| builder).await;
    ApiResultWrapper(systems_response).respond_async(
        |systems| async move {
            let groups_response: GetGroupsApiResult = requester.request(Method::GET, "/api/v1/clients/groups", Some(&mut session), |builder| builder).await;
            ApiResultWrapper(groups_response).respond(
                |groups| {
                    TeraTemplate::new(tera, PANEL_STATS_LIST_TEMPLATE, {
                        let mut ctx = Context::new();
                        ctx.insert("systems", &systems);
                        ctx.insert("groups", &groups);
                        ctx
                    }).into_response()
                }, |_err: ()| InternalError.into_response()
            )
        }, |_err: ()| async { InternalError.into_response() }
    ).await
}