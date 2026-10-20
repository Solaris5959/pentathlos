use crate::common::TestVault; // Adjust if common is loaded differently in your test tree
use pentathlos::tools::{ListFilesParams, list};

#[tokio::test]
async fn test_list_files_returns_all_markdown_uris() {
    let vault = TestVault::new();
    let params = ListFilesParams {};

    let result = list(params, &vault.path())
        .await
        .expect("Failed to list files");

    assert!(result.contains("vault://Home.md"));
    assert!(result.contains("vault://Projects/Rust_Server.md"));
    assert!(!result.contains("workspace"));
    assert!(!result.contains(".obsidian"));
}
