use crate::common::TestVault;
use std::fs;

// Assuming your function and params are exported from here
use pentathlos::tools::{FilenameSearchParams, filename_search};

#[tokio::test]
async fn test_filename_search_exact_match() {
    let vault = TestVault::new();

    // Leverage the default "Home.md" seeded in TestVault
    let params = FilenameSearchParams {
        filename: "Home.md".to_string(),
        directory: None,
    };

    let result = filename_search(params, &vault.path()).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Home.md"));
    assert!(output.contains("# Home Note"));
}

#[tokio::test]
async fn test_filename_search_filters_by_subdirectory() {
    let vault = TestVault::new();

    // TestVault already creates Projects/Rust_Server.md with "# Rust Server".
    // Let's create a duplicate file in the root to ensure filtering works.
    fs::write(vault.path().join("Rust_Server.md"), "Root level copy").unwrap();

    let params = FilenameSearchParams {
        filename: "Rust_Server.md".to_string(),
        directory: Some("Projects".to_string()),
    };

    let result = filename_search(params, &vault.path()).await.unwrap();

    // Should ONLY return the original one in the Projects folder
    assert!(result.contains("# Rust Server"));
    assert!(!result.contains("Root level copy"));
}

#[tokio::test]
async fn test_filename_search_regex_multiple_matches() {
    let vault = TestVault::new();

    // We still inject these specific files because the default vault
    // lacks a good regex pattern group, but we rely on TestVault for the environment.
    fs::write(vault.path().join("Daily_2023-10-01.md"), "Day 1").unwrap();
    fs::write(vault.path().join("Daily_2023-10-02.md"), "Day 2").unwrap();

    let params = FilenameSearchParams {
        filename: r"Daily_\d{4}-\d{2}-\d{2}\.md".to_string(),
        directory: None,
    };

    let result = filename_search(params, &vault.path()).await.unwrap();

    assert!(result.contains("Daily_2023-10-01.md"));
    assert!(result.contains("Day 1"));
    assert!(result.contains("Daily_2023-10-02.md"));
    assert!(result.contains("Day 2"));

    // Make sure it doesn't accidentally pick up Home.md
    assert!(!result.contains("Home Note"));
}

#[tokio::test]
async fn test_filename_search_no_results() {
    let vault = TestVault::new();
    // The vault is already populated with Home.md and Projects/, so we
    // don't need to manually create an "Existing.md" to prove it doesn't find it.

    let params = FilenameSearchParams {
        filename: "Nonexistent.md".to_string(),
        directory: None,
    };

    let result = filename_search(params, &vault.path()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "No matching files found.");
}

#[tokio::test]
async fn test_filename_search_invalid_regex_fails() {
    let vault = TestVault::new();

    let params = FilenameSearchParams {
        filename: "[Unclosed group".to_string(), // Invalid regex
        directory: None,
    };

    let result = filename_search(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to parse filename"));
}

#[tokio::test]
async fn test_filename_search_security_prevents_directory_traversal() {
    let vault = TestVault::new();

    let params = FilenameSearchParams {
        filename: ".*".to_string(),
        directory: Some("../../../etc".to_string()),
    };

    let result = filename_search(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Path must be relative and within the vault")
    );
}

#[tokio::test]
async fn test_filename_search_security_prevents_absolute_paths() {
    let vault = TestVault::new();

    let params = FilenameSearchParams {
        filename: ".*".to_string(),
        directory: Some("/usr/bin".to_string()), // Or "C:\\Windows"
    };

    let result = filename_search(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Path must be relative and within the vault")
    );
}

#[tokio::test]
async fn test_filename_search_missing_subdirectory_fails_gracefully() {
    let vault = TestVault::new();

    let params = FilenameSearchParams {
        filename: ".*".to_string(),
        directory: Some("GhostFolder".to_string()),
    };

    let result = filename_search(params, &vault.path()).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist in the vault"));
}
