mod common;

use common::TestVault;
use pentathlos::tools::echo;
use pentathlos::tools::files::{self, ListFilesParams, ReadFileParams, ReadFilesParams};

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

#[tokio::test]
async fn test_list_files_returns_all_markdown_uris() {
    let vault = TestVault::new();
    let params = ListFilesParams {};

    // Passing vault.path() directly ensures parallel safety
    let result = files::list_files(params, &vault.path())
        .await
        .expect("Failed to list files");

    assert!(result.contains("vault://Home.md"));
    assert!(result.contains("vault://Projects/Rust_Server.md"));
    assert!(!result.contains("workspace"));
    assert!(!result.contains(".obsidian"));
}

#[tokio::test]
async fn test_read_file_returns_correct_content() {
    let vault = TestVault::new();
    let params = ReadFileParams {
        uri: "vault://Home.md".to_string(),
    };

    let result = files::read_file(params, &vault.path())
        .await
        .expect("Failed to read file");

    assert_eq!(result, "# Home Note");
}

#[tokio::test]
async fn test_read_file_returns_not_found_error() {
    let vault = TestVault::new();
    let params = ReadFileParams {
        uri: "vault://Missing.md".to_string(),
    };

    let result = files::read_file(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No such file"));
}

// #[tokio::test]
// async fn test_read_file_blocks_path_traversal() { test for path traversal prevention, to be implemented in the future
//     let vault = TestVault::new();
//     let params = ReadFileParams {
//         uri: "vault://../outside.md".to_string(),
//     };

//     let result = files::read_file(params, &vault.path()).await;

//     assert!(result.is_err());
//     assert_eq!(result.unwrap_err(), "Path traversal blocked");
// }

#[tokio::test]
async fn test_read_multiple_files_handles_mixed_results() {
    let vault = TestVault::new();
    let params = ReadFilesParams {
        uris: vec![
            "vault://Home.md".to_string(),
            "vault://Missing.md".to_string(),
        ],
    };

    let result = files::read_files(params, &vault.path())
        .await
        .expect("Batch read failed");

    println!("Batch read result:\n{}", result);

    assert!(result.contains("--- Content of vault://Home.md ---"));
    assert!(result.contains("# Home Note"));
    assert!(result.contains("--- Content of vault://Missing.md ---"));
    assert!(result.contains("Error reading file: No such file or directory (os error 2)"));
}
