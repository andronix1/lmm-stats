use common::{result::ApiResult, systems::create::{CreateSystemApiError, CreateSystemApiRequest, CreateSystemApiResponse, CreateSystemApiResult}};
use feature_systems::domain::{models::CreateSystemInfo, repo::SystemsRepo};
use uuid::Uuid;

pub struct CreateSystemService;

impl CreateSystemService {
    pub async fn create(&self, repo: &mut dyn SystemsRepo, owner: Uuid, request: CreateSystemApiRequest) -> CreateSystemApiResult {
        if repo.name_exists(&request.name).await? {
            return ApiResult::Error(CreateSystemApiError::NameNotUnique);
        }
        if repo.human_name_exists(&request.human_name).await? {
            return ApiResult::Error(CreateSystemApiError::HumanNameNotUnique);
        }
        repo.create(CreateSystemInfo {
            name: request.name,
            human_name: request.human_name,
            owner: owner,
            secret: request.secret,
        }).await?;
        ApiResult::Success(CreateSystemApiResponse)
    }
}