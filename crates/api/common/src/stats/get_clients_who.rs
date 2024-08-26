use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClientsWhoApiRequest {
    pub events: Vec<String>,
    pub group_id: Uuid,
    pub before: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>
}

pub type GetClientsWhoApiResponse = Vec<String>;

pub type GetClientsWhoApiResult = ApiResult<GetClientsWhoApiResponse, ()>;