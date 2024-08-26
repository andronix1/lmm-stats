use abstract_sqlx_bindings::SqlxTransaction;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::{models::CreateEventCallInfo, repo::EventCallsRepo};

#[async_trait]
impl EventCallsRepo for SqlxTransaction<'_> {
    async fn add(&mut self, call: CreateEventCallInfo) -> anyhow::Result<()> {
        sqlx::query!("insert into event_calls (system, event_name, from_client) values ($1, $2, $3)", call.system, call.event, call.from_client)
            .execute(self.as_conn())
            .await?;
        Ok(())
    }

    async fn get_clients_who(&mut self, system: &str, group_id: Uuid, events: Vec<String>, after: Option<DateTime<Utc>>, before: Option<DateTime<Utc>>) -> anyhow::Result<Vec<String>> {
        Ok(sqlx::query!("select clients.name, count(clients.name) as count from event_calls inner join clients on event_calls.from_client = clients.id where system = $1 and clients.group_id = $2 and event_calls.event_name = any($3) and ($6 or at >= $4) and at <= $5 group by clients.name", system, group_id, &events, after, before.unwrap_or(Utc::now()), after.is_none())
            .fetch_all(self.as_conn())
            .await?
            .into_iter()
            .filter(|e| e.count.unwrap_or(0) >= events.len() as i64)
            .map(|e| e.name)
            .collect())
    }
}