use serde::{Deserialize, Serialize};
use crate::result::ApiResult;
use super::StatsAuthType;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StatsGetAuthInfoApiError {
    InvalidSystem
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsGetAuthInfoApiRequest {
    pub system: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsGetAuthInfoApiResponse {
    pub auth_type: StatsAuthType
}

pub type StatsGetAuthInfoApiResult = ApiResult<StatsGetAuthInfoApiResponse, StatsGetAuthInfoApiError>;