use serde::{Deserialize, Serialize};

use crate::{auth::UserRole, result::ApiResult};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum CreateUserApiError {
    AlreasyExists
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserApiRequest {
    pub login: String,
    pub password: String,
    pub role: UserRole
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserApiResponse;

pub type CreateUserApiResult = ApiResult<CreateUserApiResponse, CreateUserApiError>;