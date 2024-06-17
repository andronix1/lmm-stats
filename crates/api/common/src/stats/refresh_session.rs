use crate::{tokens::TokensPair, result::ApiResult};

pub type StatsRefreshSessionApiError = ();

pub type StatsRefreshSessionApiRequest = TokensPair;

pub type StatsRefreshSessionApiResponse = TokensPair;

pub type StatsRefreshSessionApiResult = ApiResult<StatsRefreshSessionApiResponse, StatsRefreshSessionApiError>;