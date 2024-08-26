use common::{clients::get_groups::{GetGroupsApiResult, GetGroupsGroupInfo}, result::ApiResult};
use feature_clients::domain::repo::ClientsRepo;

pub struct GetGroupsService;

impl GetGroupsService {
    pub async fn get_groups(&self, repo: &mut dyn ClientsRepo) -> GetGroupsApiResult {
        ApiResult::Success(repo.get_groups().await?.into_iter().map(|e| GetGroupsGroupInfo {
            id: e.id,
            name: e.name,
        }).collect())
    }
}