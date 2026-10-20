use crate::common::TestVault;
use std::fs::{self};

use pentathlos::tools::{
    AppendFileParams, CreateFileParams, DeleteFileParams, EditFileParams, ReadFileParams,
    ReadFilesParams, append, create, delete, edit, read_file, read_files,
};

#[tokio::test]
async fn test_read_file_returns_correct_content() {
    let vault = TestVault::new();
    let params = ReadFileParams {
        uri: "vault://Home.md".to_string(),
    };

    let result = read_file(params, &vault.path())
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

    let result = read_file(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No such file"));
}

#[tokio::test]
async fn test_read_multiple_files_handles_mixed_results() {
    let vault = TestVault::new();
    let params = ReadFilesParams {
        uris: vec![
            "vault://Home.md".to_string(),
            "vault://Missing.md".to_string(),
        ],
    };

    let result = read_files(params, &vault.path())
        .await
        .expect("Batch read failed");

    assert!(result.contains("--- Content of vault://Home.md ---"));
    assert!(result.contains("# Home Note"));
    assert!(result.contains("--- Content of vault://Missing.md ---"));
    assert!(result.contains("Error reading file: No such file or directory"));
}

#[tokio::test]
async fn test_create_file_success_root() {
    let vault = TestVault::new();
    let params = CreateFileParams {
        uri: "vault://NewNote.md".to_string(),
        content: "# New Note\nContent here.".to_string(),
    };

    let result = create(params, &vault.path()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Successfully created vault://NewNote.md");

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

    let result = create(params, &vault.path()).await;

    assert!(result.is_ok());
    let file_path = vault.path().join("Deeply/Nested/Folder/NewNote.md");
    assert!(file_path.exists());
    assert_eq!(fs::read_to_string(file_path).unwrap(), "Deep content");
}

#[tokio::test]
async fn test_create_file_fails_if_already_exists() {
    let vault = TestVault::new();
    let params = CreateFileParams {
        uri: "vault://Home.md".to_string(),
        content: "Attempted overwrite".to_string(),
    };

    let result = create(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File already exists"));

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

    let result = create(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("URI must start with vault://"));
}

#[tokio::test]
async fn test_create_file_fails_parent_is_a_file() {
    let vault = TestVault::new();
    let params = CreateFileParams {
        uri: "vault://Home.md/InvalidChild.md".to_string(),
        content: "Content".to_string(),
    };

    let result = create(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Failed to create parent directories")
    );
}

#[tokio::test]
async fn test_delete_file_success() {
    let vault = TestVault::new();
    let file_path = vault.path().join("Home.md");
    assert!(file_path.exists());

    let params = DeleteFileParams {
        uri: "vault://Home.md".to_string(),
    };

    let result = delete(params, &vault.path()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Successfully deleted vault://Home.md");
    assert!(!file_path.exists());
}

#[tokio::test]
async fn test_delete_file_fails_if_not_exists() {
    let vault = TestVault::new();
    let params = DeleteFileParams {
        uri: "vault://MissingNote.md".to_string(),
    };

    let result = delete(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File does not exists"));
}

#[tokio::test]
async fn test_delete_file_fails_invalid_uri() {
    let vault = TestVault::new();
    let params = DeleteFileParams {
        uri: "invalid://Home.md".to_string(),
    };

    let result = delete(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("URI must start with vault://"));
}

#[tokio::test]
async fn test_delete_file_fails_when_deleting_directory() {
    let vault = TestVault::new();
    let params = DeleteFileParams {
        uri: "vault://Projects".to_string(),
    };

    let result = delete(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to delete file"));
    assert!(vault.path().join("Projects").is_dir());
}

#[tokio::test]
async fn test_append_file_success() {
    let vault = TestVault::new();
    let file_path = vault.path().join("Home.md");

    let params = AppendFileParams {
        uri: "vault://Home.md".to_string(),
        new_content: "Appended content.".to_string(),
    };

    let result = append(params, &vault.path()).await;

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Successfully appended content to vault://Home.md"
    );

    let content = fs::read_to_string(file_path).unwrap();
    assert_eq!(content, "# Home Note\nAppended content.");
}

#[tokio::test]
async fn test_append_file_fails_if_not_exists() {
    let vault = TestVault::new();
    let params = AppendFileParams {
        uri: "vault://MissingNote.md".to_string(),
        new_content: "Will fail".to_string(),
    };

    let result = append(params, &vault.path()).await;
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("File does not exists") || error_msg.contains("No such file"));
}

#[tokio::test]
async fn test_append_file_fails_invalid_uri() {
    let vault = TestVault::new();
    let params = AppendFileParams {
        uri: "invalid://Home.md".to_string(),
        new_content: "Content".to_string(),
    };

    let result = append(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("URI must start with vault://"));
}

#[tokio::test]
async fn test_modify_file_success_same_line_count() {
    let vault = TestVault::new();
    let params_create = CreateFileParams {
        uri: "vault://ModifyTest.md".to_string(),
        content: "Line 1\nLine 2\nLine 3\nLine 4".to_string(),
    };
    create(params_create, &vault.path()).await.unwrap();

    let params = EditFileParams {
        uri: "vault://ModifyTest.md".to_string(),
        start_line: 2,
        end_line: 3,
        new_content: "New Line 2\nNew Line 3".to_string(),
    };

    let result = edit(params, &vault.path()).await;
    assert!(result.is_ok());

    let content = fs::read_to_string(vault.path().join("ModifyTest.md")).unwrap();
    assert_eq!(content, "Line 1\nNew Line 2\nNew Line 3\nLine 4\n");
}

#[tokio::test]
async fn test_modify_file_success_insert_more_lines() {
    let vault = TestVault::new();
    let params_create = CreateFileParams {
        uri: "vault://ExpandTest.md".to_string(),
        content: "Line 1\nLine 2\nLine 3\n".to_string(),
    };
    create(params_create, &vault.path()).await.unwrap();

    let params = EditFileParams {
        uri: "vault://ExpandTest.md".to_string(),
        start_line: 2,
        end_line: 2,
        new_content: "Inserted A\nInserted B\nInserted C".to_string(),
    };

    edit(params, &vault.path()).await.unwrap();

    let content = fs::read_to_string(vault.path().join("ExpandTest.md")).unwrap();
    assert_eq!(
        content,
        "Line 1\nInserted A\nInserted B\nInserted C\nLine 3\n"
    );
}

#[tokio::test]
async fn test_modify_file_success_remove_lines() {
    let vault = TestVault::new();
    let params_create = CreateFileParams {
        uri: "vault://ShrinkTest.md".to_string(),
        content: "Line 1\nLine 2\nLine 3\nLine 4".to_string(),
    };
    create(params_create, &vault.path()).await.unwrap();

    let params = EditFileParams {
        uri: "vault://ShrinkTest.md".to_string(),
        start_line: 2,
        end_line: 4,
        new_content: "Consolidated Line".to_string(),
    };

    edit(params, &vault.path()).await.unwrap();

    let content = fs::read_to_string(vault.path().join("ShrinkTest.md")).unwrap();
    assert_eq!(content, "Line 1\nConsolidated Line\n");
}

#[tokio::test]
async fn test_modify_file_out_of_bounds() {
    let vault = TestVault::new();
    let params_create = CreateFileParams {
        uri: "vault://BoundsTest.md".to_string(),
        content: "Line 1\nLine 2".to_string(),
    };
    create(params_create, &vault.path()).await.unwrap();

    let params_reversed = EditFileParams {
        uri: "vault://BoundsTest.md".to_string(),
        start_line: 2,
        end_line: 1,
        new_content: "Invalid".to_string(),
    };
    let result = edit(params_reversed, &vault.path()).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Bounds error"));

    let params_overflow = EditFileParams {
        uri: "vault://BoundsTest.md".to_string(),
        start_line: 1,
        end_line: 10,
        new_content: "Replaced entirely".to_string(),
    };
    let result_overflow = edit(params_overflow, &vault.path()).await;
    assert!(result_overflow.is_ok());

    let content = fs::read_to_string(vault.path().join("BoundsTest.md")).unwrap();
    assert_eq!(content, "Replaced entirely\n");
}

#[tokio::test]
async fn test_modify_file_fails_if_not_exists() {
    let vault = TestVault::new();
    let params = EditFileParams {
        uri: "vault://MissingNote.md".to_string(),
        start_line: 1,
        end_line: 1,
        new_content: "Will fail".to_string(),
    };

    let result = edit(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to read file"));
}
