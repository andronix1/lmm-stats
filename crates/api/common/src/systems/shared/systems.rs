use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoApiResponse {
    pub human_name: String,
    pub active: bool,
    pub owner_login: String,
    pub secret: Option<String>,
}