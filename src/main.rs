mod models;
mod prompts;
mod resources;
mod server;
mod services;
mod tools;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = server::ServerResourceConfig::from_env();
    server::start(config).await?;

    Ok(())
}
