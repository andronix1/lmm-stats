pub mod jwt;

#[derive(Debug)]
pub enum TokenVerifyError {
    Expired
}

pub trait TokenGenerator<T> {
    fn generate(&self, claims: T) -> anyhow::Result<String>;
    fn verify(&self, token: String) -> anyhow::Result<Result<T, TokenVerifyError>>;
    fn extract_payload(&self, token: String) -> anyhow::Result<T>;
}