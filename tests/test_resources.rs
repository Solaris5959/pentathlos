mod common;

use common::TestVault;
use pentathlos::resources::vault::ResourceManager; 

#[test]
fn test_manager_canonicalizes_path() {
    let vault = TestVault::new();
    // This will fail if canonicalize() breaks, proving the test setup is valid
    let manager = ResourceManager::new(vault.path()).expect("Failed to init manager");
    
    assert!(manager.list_resources().len() == 1);
}

#[test]
fn test_list_resources_returns_index_uri() {
    let vault = TestVault::new();
    let manager = ResourceManager::new(vault.path()).unwrap();
    
    let resources = manager.list_resources();
    assert_eq!(resources.len(), 1);
    
    let index_resource = &resources[0];
    assert_eq!(index_resource.uri, "vault://index");
    assert_eq!(index_resource.mime_type.as_deref(), Some("text/plain"));
}

#[test]
fn test_read_resource_generates_correct_markdown_index() {
    let vault = TestVault::new();
    let manager = ResourceManager::new(vault.path()).unwrap();
    
    // Read the vault://index resource
    let content = manager.read_resource("vault://index").expect("Failed to read index");
    
    assert_eq!(content.uri, "vault://index");
    assert_eq!(content.mime_type.as_deref(), Some("text/plain"));
    
    let text = content.text;
    
    // Verify header exists
    assert!(text.contains("# Obsidian Vault Index"));
    
    // Verify standard and nested files are found
    // The replace("\\", "/") in your logic ensures we can safely check for forward slashes
    assert!(text.contains("- Home.md"));
    assert!(text.contains("- Projects/Rust_Server.md"));
    
    // Verify .obsidian files are explicitly ignored
    assert!(!text.contains("workspace"));
    assert!(!text.contains(".obsidian"));
}

#[test]
fn test_read_resource_returns_not_found_for_invalid_uri() {
    let vault = TestVault::new();
    let manager = ResourceManager::new(vault.path()).unwrap();
    
    let result = manager.read_resource("vault://nonexistent");
    
    assert!(result.is_err());
    match result {
        Err(e) => assert_eq!(e.to_string(), "Resource not found: vault://nonexistent"),
        Ok(_) => panic!("Expected NotFound error"),
    }
}