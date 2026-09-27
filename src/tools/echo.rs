use rmcp::ErrorData;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EchoParams {
    pub message: String,
}

pub async fn run(args: EchoParams) -> Result<String, ErrorData> {
    Ok(format!("Server successfully received: {}", args.message))
}
