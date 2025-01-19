use berry::setup_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_server().await?.run().await
}
