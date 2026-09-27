mod server;
mod models;
mod prompts;
mod resources;
mod services;
mod tools;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::start().await?;

    Ok(())
}