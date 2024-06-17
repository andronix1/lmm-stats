use serde::{Deserialize, Serialize};

use crate::result::ApiResult;


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SystemEventCreateApiError {
    NameNotUnique,
    HumanNameNotUnique,
    NotFound
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemEventCreateApiRequest {
    pub name: String,
    pub human_name: String
}

pub type SystemEventCreateApiResponse = ();

pub type SystemEventCreateApiResult = ApiResult<SystemEventCreateApiResponse, SystemEventCreateApiError>;