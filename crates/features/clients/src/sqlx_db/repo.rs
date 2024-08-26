use abstract_sqlx_bindings::SqlxTransaction;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{models::{ClientInfo, FullGroup, GroupInfo}, repo::ClientsRepo};

#[async_trait]
impl ClientsRepo for SqlxTransaction<'_> {
    async fn has_group_name(&mut self, name: &str) -> anyhow::Result<bool> {
        Ok(sqlx::query!("select count(*) from client_groups where name = $1", name)
            .fetch_one(self.as_conn())
            .await?.count == Some(1)
        )
    }

    async fn has_group_id(&mut self, id: Uuid) -> anyhow::Result<bool> {
        Ok(sqlx::query!("select count(*) from client_groups where id = $1", id)
            .fetch_one(self.as_conn())
            .await?.count == Some(1)
        )
    }

    async fn create_group(&mut self, name: &str) -> anyhow::Result<()> {
        sqlx::query!("insert into client_groups (name) values ($1)", name)
            .execute(self.as_conn())
            .await?;
        Ok(())
    }

    async fn merge_clients(&mut self, group_id: Uuid, clients: Vec<String>) -> anyhow::Result<()> {
        sqlx::query!("insert into clients (group_id, name) select $1, client from unnest($2::text[]) as a(client) on conflict (name) do update set group_id = $1", group_id, clients.as_slice())
            .execute(self.as_conn())
            .await?;
        Ok(())
    }

    async fn get_full_groups(&mut self) -> anyhow::Result<Vec<FullGroup>> {
        Ok(sqlx::query!("select g.id, g.name, array_remove(array_agg(c.id), null) as client_ids, array_remove(array_agg(c.name), null) as client_names from client_groups g left join clients c on c.group_id = g.id group by g.id order by g.name")
            .fetch_all(self.as_conn())
            .await?
            .into_iter()
            .map(|dto| FullGroup { 
                id: dto.id,
                name: dto.name,
                clients: dto.client_ids.unwrap_or(vec![])
                    .into_iter()
                    .zip(dto.client_names.unwrap_or(vec![]))
                    .map(|(id, name)| ClientInfo { id, name })
                    .collect()
            }).collect()
        )
    }

    async fn try_delete_group(&mut self, group_id: Uuid) -> anyhow::Result<bool> {
        Ok(sqlx::query!("delete from client_groups where id = $1", group_id)
            .execute(self.as_conn())
            .await?.rows_affected() == 1)
    }

    async fn try_delete_client(&mut self, client_id: Uuid) -> anyhow::Result<bool> {
        Ok(sqlx::query!("delete from clients where id = $1", client_id)
            .execute(self.as_conn())
            .await?.rows_affected() == 1)
    }

    async fn client_exists(&mut self, id: Uuid) -> anyhow::Result<bool> {
        Ok(sqlx::query!("select count(*) from clients where id = $1", id)
            .fetch_one(self.as_conn())
            .await?.count == Some(1)
        )
    }

    async fn get_groups(&mut self) -> anyhow::Result<Vec<GroupInfo>> {
        Ok(sqlx::query_as!(GroupInfo, "select id, name from client_groups")
            .fetch_all(self.as_conn())
            .await?
        )
    }
}