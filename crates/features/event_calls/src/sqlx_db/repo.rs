use abstract_sqlx_bindings::SqlxTransaction;
use async_trait::async_trait;

use crate::domain::{models::CreateEventCallInfo, repo::EventCallsRepo};

#[async_trait]
impl EventCallsRepo for SqlxTransaction<'_> {
    async fn add(&mut self, call: CreateEventCallInfo) -> anyhow::Result<()> {
        sqlx::query!("insert into event_calls (system, event_name, from_client) values ($1, $2, $3)", call.system, call.event, call.from_client)
            .execute(self.as_conn())
            .await?;
        Ok(())
    }
}