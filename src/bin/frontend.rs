#[tokio::main]
async fn main() -> anyhow::Result<()> {
    frontend::run().await
}