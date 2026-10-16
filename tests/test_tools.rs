mod common;

use common::TestVault;
use pentathlos::tools::echo;
use pentathlos::tools::files::{
    self, CreateFileParams, DeleteFileParams, ListFilesParams, ReadFileParams, ReadFilesParams,
};
use std::fs::{self};

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

#[tokio::test]
async fn test_create_file_success_root() {
    let vault = TestVault::new();
    let params = CreateFileParams {
        uri: "vault://NewNote.md".to_string(),
        content: "# New Note\nContent here.".to_string(),
    };

    let result = files::create_file(params, &vault.path()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Successfully created vault://NewNote.md");

    // Verify file actually exists and contains the correct data
    let file_path = vault.path().join("NewNote.md");
    assert!(file_path.exists());
    assert_eq!(
        fs::read_to_string(file_path).unwrap(),
        "# New Note\nContent here."
    );
}

#[tokio::test]
async fn test_create_file_success_nested_directories() {
    let vault = TestVault::new();
    let params = CreateFileParams {
        uri: "vault://Deeply/Nested/Folder/NewNote.md".to_string(),
        content: "Deep content".to_string(),
    };

    let result = files::create_file(params, &vault.path()).await;

    assert!(result.is_ok());

    // Verify parent directories were created
    let file_path = vault.path().join("Deeply/Nested/Folder/NewNote.md");
    assert!(file_path.exists());
    assert_eq!(fs::read_to_string(file_path).unwrap(), "Deep content");
}

#[tokio::test]
async fn test_create_file_fails_if_already_exists() {
    let vault = TestVault::new();
    // vault://Home.md is created by TestVault::new()
    let params = CreateFileParams {
        uri: "vault://Home.md".to_string(),
        content: "Attempted overwrite".to_string(),
    };

    let result = files::create_file(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("File already exists at URI: vault://Home.md")
    );

    // Verify content was NOT overwritten
    let file_path = vault.path().join("Home.md");
    assert_eq!(fs::read_to_string(file_path).unwrap(), "# Home Note");
}

#[tokio::test]
async fn test_create_file_fails_invalid_uri() {
    let vault = TestVault::new();
    let params = CreateFileParams {
        uri: "invalid://Home.md".to_string(),
        content: "Content".to_string(),
    };

    let result = files::create_file(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("URI must start with vault://"));
}

#[tokio::test]
async fn test_create_file_fails_parent_is_a_file() {
    let vault = TestVault::new();
    // Attempt to create a file where the parent path is already a file (Home.md),
    // which will trigger the `fs::create_dir_all` OS error.
    let params = CreateFileParams {
        uri: "vault://Home.md/InvalidChild.md".to_string(),
        content: "Content".to_string(),
    };

    let result = files::create_file(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Failed to create parent directories")
    );
}

// --- Delete File Tests ---

#[tokio::test]
async fn test_delete_file_success() {
    let vault = TestVault::new();
    let file_path = vault.path().join("Home.md");

    // Ensure it exists before testing
    assert!(file_path.exists());

    let params = DeleteFileParams {
        uri: "vault://Home.md".to_string(),
    };

    let result = files::delete_file(params, &vault.path()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Successfully deleted vault://Home.md");

    // Verify it was actually removed from the file system
    assert!(!file_path.exists());
}

#[tokio::test]
async fn test_delete_file_fails_if_not_exists() {
    let vault = TestVault::new();
    let params = DeleteFileParams {
        uri: "vault://MissingNote.md".to_string(),
    };

    let result = files::delete_file(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("File does not exists at URI: vault://MissingNote.md")
    );
}

#[tokio::test]
async fn test_delete_file_fails_invalid_uri() {
    let vault = TestVault::new();
    let params = DeleteFileParams {
        uri: "invalid://Home.md".to_string(),
    };

    let result = files::delete_file(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("URI must start with vault://"));
}

#[tokio::test]
async fn test_delete_file_fails_when_deleting_directory() {
    let vault = TestVault::new();
    // Projects is a directory created by TestVault::new()
    let params = DeleteFileParams {
        uri: "vault://Projects".to_string(),
    };

    let result = files::delete_file(params, &vault.path()).await;

    // fs::remove_file fails when called on a directory (requires fs::remove_dir)
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to delete file"));

    // Verify the directory is still there
    assert!(vault.path().join("Projects").is_dir());
}
