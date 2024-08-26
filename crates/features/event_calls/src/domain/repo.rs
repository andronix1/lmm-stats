use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::models::CreateEventCallInfo;

#[async_trait]
pub trait EventCallsRepo : Send {
    async fn add(&mut self, call: CreateEventCallInfo) -> anyhow::Result<()>;
    async fn get_clients_who(&mut self, system: &str, group_id: Uuid, events: Vec<String>, after: Option<DateTime<Utc>>, before: Option<DateTime<Utc>>) -> anyhow::Result<Vec<String>>;
}