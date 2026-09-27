use pentathlos::tools::echo;

#[tokio::test]
async fn test_echo_tool() {
    let params = echo::EchoParams {
        message: "Hello, World!".to_string(),
    };

    // Await the future before calling unwrap()
    let result = echo::run(params).await;

    // Update the assertion to match your expected server response format
    assert_eq!(
        result.unwrap(),
        "Server successfully received: Hello, World!"
    );
}
