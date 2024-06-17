use abstract_sqlx_bindings::SqlxTransaction;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{models::{CreateSystemInfo, FullSystemInfo, ShortSystemInfo, SystemPatch}, repo::SystemsRepo};

#[async_trait]
impl SystemsRepo for SqlxTransaction<'_> {
    async fn create(&mut self, info: CreateSystemInfo) -> anyhow::Result<()> {
        sqlx::query!("insert into systems (name, human_name, owner, secret) values ($1, $2, $3, $4)", info.name, info.human_name, info.owner, info.secret)
            .execute(self.as_conn())
            .await?;
        Ok(())
    }

    async fn change_secret(&mut self, name: &str, secret: Option<&str>) -> anyhow::Result<()> {
        sqlx::query!("update systems set secret = $2 where name = $1", name, secret)
            .execute(self.as_conn())
            .await?;
        Ok(())
    }
    
    async fn delete(&mut self, name: &str) -> anyhow::Result<()> {
        sqlx::query!("delete from systems where name = $1", name)
            .execute(self.as_conn())
            .await?;
        Ok(())
    }

    async fn get_owned(&mut self, owner: Uuid) -> anyhow::Result<Vec<ShortSystemInfo>> {
        Ok(sqlx::query_as!(ShortSystemInfo, "select name, human_name from systems where owner = $1", owner)
            .fetch_all(self.as_conn())
            .await?
        )
    }

    async fn name_exists(&mut self, name: &str) -> anyhow::Result<bool> {
        Ok(sqlx::query!("select count(*) from systems where name = $1", name)
            .fetch_one(self.as_conn())
            .await?.count == Some(1)
        )
    }

    async fn human_name_exists(&mut self, human_name: &str) -> anyhow::Result<bool> {
        Ok(sqlx::query!("select count(*) from systems where human_name = $1", human_name)
            .fetch_one(self.as_conn())
            .await?.count == Some(1)
        )
    }

    async fn try_delete_owned(&mut self, name: &str, owner: Uuid) -> anyhow::Result<bool> {
        Ok(sqlx::query!("delete from systems where name = $1 and owner = $2", name, owner)
            .execute(self.as_conn())
            .await?.rows_affected() == 1
        )
    }

    async fn try_get_owned_info(&mut self, name: &str, owner: Uuid) -> anyhow::Result<Option<FullSystemInfo>> {
        Ok(sqlx::query_as!(FullSystemInfo, "select human_name, active, users.login as owner_login, secret from systems inner join users on users.id = systems.owner where name = $1 and owner = $2", name, owner)
            .fetch_optional(self.as_conn())
            .await?
        )
    }

    async fn try_get_info(&mut self, name: &str) -> anyhow::Result<Option<FullSystemInfo>> {
        Ok(sqlx::query_as!(FullSystemInfo, "select human_name, active, users.login as owner_login, secret from systems inner join users on users.id = systems.owner where name = $1", name)
            .fetch_optional(self.as_conn())
            .await?
        )
    }

    async fn try_patch(&mut self, name: &str, owner: Uuid, patch: SystemPatch) -> anyhow::Result<bool> {
        Ok(sqlx::query!("update systems set active = coalesce($3, active), human_name = coalesce($4, human_name), secret = case when $5 then $6 else secret end where name = $1 and owner = $2", 
                name, owner, patch.active, patch.human_name, 
                patch.secret.is_some(),
                if let Some(secret) = patch.secret { secret } else { None }
            )
            .execute(self.as_conn())
            .await?.rows_affected() == 1)
    }

    async fn try_get_name_by_human_name(&mut self, human_name: &str) -> anyhow::Result<Option<String>> {
        Ok(sqlx::query!("select name from systems where human_name = $1", human_name)
            .fetch_optional(self.as_conn())
            .await?
            .map(|dto| dto.name)
        )
    }

    async fn has_owned(&mut self, name: &str, owner: Uuid) -> anyhow::Result<bool> {
        Ok(sqlx::query!("select count(*) from systems where name = $1 and owner = $2", name, owner)
            .fetch_one(self.as_conn())
            .await?.count == Some(1)
        )
    }

    async fn get_secret(&mut self, name: &str) -> anyhow::Result<Option<String>> {
        Ok(sqlx::query!("select secret from systems where name = $1", name)
            .fetch_one(self.as_conn())
            .await?.secret
        )
    }

    async fn has_secret(&mut self, name: &str) -> anyhow::Result<bool> {
        Ok(sqlx::query!("select secret is null as has_secret from systems where name = $1", name)
            .fetch_one(self.as_conn())
            .await?.has_secret == Some(true)
        )
    }
}