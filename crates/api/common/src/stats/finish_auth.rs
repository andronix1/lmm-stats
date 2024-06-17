use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::result::ApiResult;

pub type  StatsFinishAuthApiError = ();

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsFinishAuthApiRequest {
    pub client_id: Uuid
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsFinishAuthApiResponse {
    pub access_token: String
}

pub type StatsFinishAuthApiResult = ApiResult<StatsFinishAuthApiResponse, StatsFinishAuthApiError>;