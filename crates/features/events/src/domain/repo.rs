use async_trait::async_trait;
use uuid::Uuid;

use super::models::{CreateEventInfo, ShortEventInfo};

#[async_trait]
pub trait EventsRepo {
    async fn create(&mut self, info: CreateEventInfo) -> anyhow::Result<()>;
    async fn get_owned(&mut self, owner: Uuid) -> anyhow::Result<Vec<ShortEventInfo>>;
    async fn exists(&mut self, system: &str, name: &str) -> anyhow::Result<bool>;
    async fn system_human_name_exists(&mut self, system: &str, human_name: &str) -> anyhow::Result<bool>;
    async fn try_delete_owned(&mut self, system: &str, name: &str, owner: Uuid) -> anyhow::Result<bool>;
}