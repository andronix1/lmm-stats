use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct GetSystemsListEventInfo {
    pub name: String,
    pub human_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct GetSystemsListSystemInfo {
    pub name: String,
    pub human_name: String,
    pub activated_at: DateTime<Utc>,
    pub active: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct GetSystemsListFullSystemInfo {
    #[serde(flatten)]
    pub system: GetSystemsListSystemInfo,
    pub events: Vec<GetSystemsListEventInfo>
}

pub type GetSystemsListApiResult = ApiResult<Vec<GetSystemsListFullSystemInfo>, ()>;