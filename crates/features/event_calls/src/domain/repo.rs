use async_trait::async_trait;

use super::models::CreateEventCallInfo;

#[async_trait]
pub trait EventCallsRepo {
    async fn add(&mut self, call: CreateEventCallInfo) -> anyhow::Result<()>;
}