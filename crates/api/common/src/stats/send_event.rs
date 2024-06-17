use serde::{Deserialize, Serialize};
use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StatsSendEventApiError {
    InvalidEvent
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsSendEventApiRequest {
    pub name: String
}

pub type StatsSendEventApiResponse = ();

pub type StatsSendEventApiResult = ApiResult<StatsSendEventApiResponse, StatsSendEventApiError>;