use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::result::ApiResult;

pub type StatsGetSessionInfoApiError = ();

pub type StatsGetSessionInfoApiRequest = ();

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatsGetSessionInfoApiResponse {
    pub client_id: Option<Uuid>,
    pub system: String,
}

pub type StatsGetSessionInfoApiResult = ApiResult<StatsGetSessionInfoApiResponse, StatsGetSessionInfoApiError>;