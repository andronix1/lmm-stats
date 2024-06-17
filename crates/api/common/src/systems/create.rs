use serde::{Deserialize, Serialize};

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CreateSystemApiError {
    HumanNameNotUnique,
    NameNotUnique
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSystemApiRequest {
    pub name: String,
    pub human_name: String,
    #[serde(default)]
    pub secret: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct CreateSystemApiResponse;

pub type CreateSystemApiResult = ApiResult<CreateSystemApiResponse, CreateSystemApiError>;