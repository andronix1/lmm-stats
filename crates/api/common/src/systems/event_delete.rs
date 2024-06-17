use serde::{Deserialize, Serialize};

use crate::result::ApiResult;


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SystemEventDeleteApiError {
    NotFound
}

pub type SystemEventDeleteApiRequest = ();

pub type SystemEventDeleteApiResponse = ();

pub type SystemEventDeleteApiResult = ApiResult<SystemEventDeleteApiResponse, SystemEventDeleteApiError>;