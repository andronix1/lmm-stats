use serde::{Deserialize, Serialize};
use crate::result::ApiResult;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetEventsOfOwnedApiError {
    NotFound
}

pub type GetEventsOfOwnedApiRequest = ();

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventsOfOwnedApiEventInfo {
    pub name: String,
    pub human_name: String
}

pub type GetEventsOfOwnedApiResponse = Vec<GetEventsOfOwnedApiEventInfo>;

pub type GetEventsOfOwnedApiResult = ApiResult<GetEventsOfOwnedApiResponse, GetEventsOfOwnedApiError>;