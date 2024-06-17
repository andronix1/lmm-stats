use serde::{Deserialize, Serialize};

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOwnedSystemsApiSystemInfo {
    pub name: String,
    pub human_name: String,
}

pub type GetOwnedSystemsApiResponse = Vec<GetOwnedSystemsApiSystemInfo>;

pub type GetOwnedSystemsApiResult = ApiResult<GetOwnedSystemsApiResponse, ()>;