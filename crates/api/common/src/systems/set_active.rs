use serde::{Deserialize, Serialize};

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SetSystemActiveError {
    SystemNotExists
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSystemActiveRequest {
    pub name: String,
    pub active: bool
}

pub type SetSystemActiveApiResult = ApiResult<(), SetSystemActiveError>;