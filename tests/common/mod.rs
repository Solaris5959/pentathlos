use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct TestVault {
    pub dir: TempDir,
}

impl TestVault {
    pub fn new() -> Self {
        let dir = TempDir::new().expect("Failed to create temporary vault");
        let vault_path = dir.path();

        // 1. Create a standard markdown note
        fs::write(vault_path.join("Home.md"), "# Home Note").unwrap();

        // 2. Create a nested markdown note
        let nested_dir = vault_path.join("Projects");
        fs::create_dir_all(&nested_dir).unwrap();
        fs::write(nested_dir.join("Rust_Server.md"), "# Rust Server").unwrap();

        // 3. Create an excluded hidden directory
        let obsidian_dir = vault_path.join(".obsidian");
        fs::create_dir_all(&obsidian_dir).unwrap();
        fs::write(obsidian_dir.join("workspace"), "{}").unwrap();

        Self { dir }
    }

    pub fn path(&self) -> PathBuf {
        self.dir.path().to_path_buf()
    }
}
