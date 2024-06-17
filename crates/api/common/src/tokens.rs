use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct TokensPair {
    pub access_token: String,
    pub refresh_token: String,
}