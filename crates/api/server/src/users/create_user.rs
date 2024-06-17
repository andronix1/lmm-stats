use api_common::{result::ApiResult, users::create::{CreateUserApiError, CreateUserApiRequest, CreateUserApiResponse, CreateUserApiResult}};
use feature_users::domain::{UserCreateInfo, UsersRepo};

pub struct CreateUserService;

impl CreateUserService {
    pub async fn create(&self, repo: &mut dyn UsersRepo, request: CreateUserApiRequest) -> CreateUserApiResult {
        if repo.login_exists(request.login.clone()).await? {
            return ApiResult::Error(CreateUserApiError::AlreasyExists);
        }
        UsersRepo::create(repo, UserCreateInfo {
            login: request.login,
            password: request.password,
            role: request.role.into(),
        }).await?;
        ApiResult::Success(CreateUserApiResponse)
    }
}