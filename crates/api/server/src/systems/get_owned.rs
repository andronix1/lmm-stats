use common::{result::ApiResult, systems::get_owned::{GetOwnedSystemsApiResult, GetOwnedSystemsApiSystemInfo}};
use feature_systems::domain::repo::SystemsRepo;
use uuid::Uuid;

pub struct GetOwnedSystemsService;

impl GetOwnedSystemsService {
    pub async fn get_owned(&self, repo: &mut dyn SystemsRepo, owner: Uuid) -> GetOwnedSystemsApiResult {
        ApiResult::Success(repo.get_owned(owner).await?.into_iter().map(|e| GetOwnedSystemsApiSystemInfo {
            name: e.name,
            human_name: e.human_name,
        }).collect())
    }
}