use common::{clients::get_clients::{GetClientsApiGroup, GetClientsApiResult}, result::ApiResult};
use feature_clients::domain::repo::ClientsRepo;
use linked_hash_map::LinkedHashMap;

pub struct GetClientsService;

impl GetClientsService {
    pub async fn get(&self, repo: &mut dyn ClientsRepo) -> GetClientsApiResult {
        let result = repo.get_full_groups().await?;
        let mut map = LinkedHashMap::new();
        for group in result {
            map.insert(group.id, GetClientsApiGroup {
                name: group.name,
                clients: {
                    let mut map = LinkedHashMap::new();
                    for client in group.clients {
                        map.insert(client.id, client.name);
                    }
                    map
                },
            });
        }
        ApiResult::Success(map)
    }
}