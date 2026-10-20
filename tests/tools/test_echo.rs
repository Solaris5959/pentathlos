use pentathlos::tools::echo;

#[tokio::test]
async fn test_echo_tool() {
    let params = echo::EchoParams {
        message: "Hello, World!".to_string(),
    };
    let result = echo::run(params).await;
    assert_eq!(
        result.unwrap(),
        "Server successfully received: Hello, World!"
    );
}
