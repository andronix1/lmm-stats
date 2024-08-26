use common::{result::ApiResult, systems::list::{GetSystemsListApiResult, GetSystemsListEventInfo, GetSystemsListFullSystemInfo, GetSystemsListSystemInfo}};
use feature_events::domain::repo::EventsRepo;
use feature_systems::domain::repo::SystemsRepo;

pub struct GetSystemsListService;

impl GetSystemsListService {
    pub async fn list<T: SystemsRepo + EventsRepo>(&self, repo: &mut T) -> GetSystemsListApiResult {
        let mut result = Vec::new();
        let systems = SystemsRepo::get_all(repo).await?;
        for system in systems {
            let events = EventsRepo::get_of_system(repo, &system.name).await?.into_iter().map(|e| GetSystemsListEventInfo {
                name: e.name,
                human_name: e.human_name
            }).collect();
            result.push(GetSystemsListFullSystemInfo {
                system: GetSystemsListSystemInfo {
                    name: system.name,
                    human_name: system.human_name,
                    active: system.active,
                    activated_at: system.activated_at,
                },
                events
            })
        }
        ApiResult::Success(result)
    }
}