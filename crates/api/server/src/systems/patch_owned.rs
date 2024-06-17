use common::{result::ApiResult, systems::patch_owned::{PatchOwnedSystemApiError, PatchOwnedSystemApiRequest, PatchOwnedSystemApiResponse, PatchOwnedSystemApiResult}};
use feature_systems::domain::{models::SystemPatch, repo::SystemsRepo};
use uuid::Uuid;

pub struct PatchOwnedSystemService;

impl PatchOwnedSystemService {
    pub async fn patch_owned(&self, repo: &mut dyn SystemsRepo, name: String, owner_id: Uuid, request: PatchOwnedSystemApiRequest) -> PatchOwnedSystemApiResult {
        if !repo.name_exists(&name).await? {
            return ApiResult::Error(PatchOwnedSystemApiError::NotExists);
        }
        if let Some(human_name) = &request.human_name {
            let name_by_human_name = repo.try_get_name_by_human_name(human_name).await?;
            if name_by_human_name.is_some() && name_by_human_name.as_ref() != Some(&name) {
                return ApiResult::Error(PatchOwnedSystemApiError::HumanNameNotUnique);
            }
        }
        if !repo.try_patch(&name, owner_id, SystemPatch {
                human_name: request.human_name,
                active: request.active,
                secret: request.secret,
            }).await? {
                return ApiResult::Forbidden;
            }
        ApiResult::Success(PatchOwnedSystemApiResponse)
    }
}