use serde::{Deserialize, Serialize};
use crate::{tokens::TokensPair, result::ApiResult};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StatsAuthApiError {
    InvalidSystem,
    InvalidAuthType,
    InvalidSecret,
    StatsNotActive
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "authType")]
pub enum StatsAuthApiRequest {
    Secret { secret: String },
    None,
}

pub type StatsAuthApiResponse = TokensPair;

pub type StatsAuthApiResult = ApiResult<StatsAuthApiResponse, StatsAuthApiError>;