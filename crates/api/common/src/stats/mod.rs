use serde::{Deserialize, Serialize};

pub mod get_session_info;
pub mod get_auth_info;
pub mod refresh_session;
pub mod send_event;
pub mod finish_auth;
pub mod get_clients_who;
pub mod auth;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StatsAuthType {
    Secret,
    None
}