use async_trait::async_trait;
use uuid::Uuid;

use super::models::FullGroup;

#[async_trait]
pub trait ClientsRepo: Send + Sync {
    async fn has_group_name(&mut self, name: &str) -> anyhow::Result<bool>;
    async fn has_group_id(&mut self, id: Uuid) -> anyhow::Result<bool>;
    async fn client_exists(&mut self, id: Uuid) -> anyhow::Result<bool>;
    async fn create_group(&mut self, name: &str) -> anyhow::Result<()>;
    async fn merge_clients(&mut self, group_id: Uuid, clients: Vec<String>) -> anyhow::Result<()>;
    async fn get_full_groups(&mut self) -> anyhow::Result<Vec<FullGroup>>;
    async fn try_delete_group(&mut self, group_id: Uuid) -> anyhow::Result<bool>;
    async fn try_delete_client(&mut self, client_id: Uuid) -> anyhow::Result<bool>;
}