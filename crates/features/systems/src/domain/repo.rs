use async_trait::async_trait;
use uuid::Uuid;

use super::models::{CreateSystemInfo, FullSystemInfo, ListSystemInfo, ShortSystemInfo, SystemPatch};

#[async_trait]
pub trait SystemsRepo: Sync + Send {
    async fn create(&mut self, info: CreateSystemInfo) -> anyhow::Result<()>;
    async fn change_secret(&mut self, name: &str, secret: Option<&str>) -> anyhow::Result<()>;
    async fn delete(&mut self, name: &str) -> anyhow::Result<()>;
    async fn get_owned(&mut self, owner: Uuid) -> anyhow::Result<Vec<ShortSystemInfo>>;
    async fn name_exists(&mut self, name: &str) -> anyhow::Result<bool>;
    async fn human_name_exists(&mut self, human_name: &str) -> anyhow::Result<bool>;
    async fn has_owned(&mut self, name: &str, owner: Uuid) -> anyhow::Result<bool>;
    async fn try_get_name_by_human_name(&mut self, human_name: &str) -> anyhow::Result<Option<String>>;
    async fn try_delete_owned(&mut self, name: &str, owner: Uuid) -> anyhow::Result<bool>;
    async fn get_secret(&mut self, name: &str) -> anyhow::Result<Option<String>>;
    async fn has_secret(&mut self, name: &str) -> anyhow::Result<bool>;
    async fn active_name_exists(&mut self, name: &str) -> anyhow::Result<bool>;
    async fn try_get_active_status(&mut self, name: &str) -> anyhow::Result<Option<bool>>;

    async fn try_get_owned_info(&mut self, name: &str, owner: Uuid) -> anyhow::Result<Option<FullSystemInfo>>;
    async fn try_get_info(&mut self, name: &str) -> anyhow::Result<Option<FullSystemInfo>>;

    async fn try_patch(&mut self, name: &str, owner: Uuid, patch: SystemPatch) -> anyhow::Result<bool>;

    async fn get_all(&mut self) -> anyhow::Result<Vec<ListSystemInfo>>;

    async fn try_set_active(&mut self, name: &str, active: bool) -> anyhow::Result<bool>;

    async fn mark_activated(&mut self, name: &str) -> anyhow::Result<()>;
}
