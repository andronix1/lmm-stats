use abstract_sqlx_bindings::SqlxTransaction;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{models::{CreateEventInfo, ShortEventInfo}, repo::EventsRepo};

#[async_trait]
impl EventsRepo for SqlxTransaction<'_> {
    async fn create(&mut self, info: CreateEventInfo) -> anyhow::Result<()> {
        sqlx::query!("insert into events (name, system, human_name) values ($1, $2, $3)", info.name, info.system, info.human_name)
            .execute(self.as_conn())
            .await?;
        Ok(())
    }

    async fn get_owned(&mut self, owner: Uuid) -> anyhow::Result<Vec<ShortEventInfo>> {
        Ok(sqlx::query_as!(ShortEventInfo, "select events.name, events.human_name from events inner join systems on systems.name = events.system where systems.owner = $1", owner)
            .fetch_all(self.as_conn())
            .await?
        )
    }

    async fn exists(&mut self, system: &str, name: &str) -> anyhow::Result<bool> {
        Ok(sqlx::query!("select count(*) from events where name = $1 and system = $2", name, system)
            .fetch_one(self.as_conn())
            .await?.count == Some(1)
        )
    }

    async fn system_human_name_exists(&mut self, system: &str, human_name: &str) -> anyhow::Result<bool> {
        Ok(sqlx::query!("select count(*) from events where human_name = $1 and system = $2", human_name, system)
            .fetch_one(self.as_conn())
            .await?.count == Some(1)
        )
    }

    async fn try_delete_owned(&mut self, system: &str, name: &str, owner: Uuid) -> anyhow::Result<bool> {
        Ok(sqlx::query!("delete from events using systems where events.name = $1 and system = $2 and systems.owner = $3", name, system, owner)
            .execute(self.as_conn())
            .await?.rows_affected() == 1)
    }
}