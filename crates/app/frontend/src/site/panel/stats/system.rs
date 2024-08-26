use std::sync::Arc;

use api_common::stats::get_clients_who::{GetClientsWhoApiRequest, GetClientsWhoApiResult};
use axum::{extract::{Path, Query}, response::{IntoResponse, Response}, Extension};
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{de, Deserialize, Deserializer};
use tera::{Context, Tera};
use uuid::Uuid;

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::Session, tera::TeraTemplate};

use super::SystemPath;

const PANEL_SYSTEM_STATS_TEMPLATE: &'static str = "panel/stats/system.tera";

fn deserialize_datetime_from_query<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let data: Option<String> = Deserialize::deserialize(deserializer)?;
    Ok(match data.map(|s| DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%SZ%z")) {
        Some(Ok(dt)) => Some(dt.into()),
        Some(Err(res)) => return Err(de::Error::custom(String::from("datetime parse error: ") + &res.to_string())),
        None => None
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClientsWhoQuery {
    pub events: String,
    pub group_id: Uuid,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_datetime_from_query")]
    pub before: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_datetime_from_query")]
    pub after: Option<DateTime<Utc>>
}

pub async fn get(
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>,
    mut session: Session,
    Path(path): Path<SystemPath>,
    Query(query): Query<GetClientsWhoQuery>,
) -> Response { 
    let result: GetClientsWhoApiResult = requester.request(Method::POST, &format!("/api/v1/stats/system/{}/clients", path.name), Some(&mut session), |builder| builder.json(&GetClientsWhoApiRequest {
        events: query.events.split(' ').map(String::from).collect(),
        group_id: query.group_id,
        before: query.before,
        after: query.after,
    })).await;
    ApiResultWrapper(result).respond(
        |clients| TeraTemplate::new(tera, PANEL_SYSTEM_STATS_TEMPLATE, {
            let mut ctx = Context::new();
            ctx.insert("clients", &clients);
            ctx
        }).into_response(), |_err: ()| unreachable!()
    )
}