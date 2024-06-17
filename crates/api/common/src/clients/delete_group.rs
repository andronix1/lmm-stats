
use serde::{Deserialize, Serialize};

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeleteGroupApiError {
    NotExists
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteGroupApiRequest {
    pub name: String
}

pub type DeleteGroupApiResponse = ();

pub type DeleteGroupApiResult = ApiResult<DeleteGroupApiResponse, DeleteGroupApiError>;