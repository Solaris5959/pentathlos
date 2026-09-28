use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

// Standard MCP Resources

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceContent {
    pub uri: String,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    pub text: String,
}

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Resource not found: {0}")]
    NotFound(String),
}

// Resource Manager

pub struct ResourceManager {
    vault_root: PathBuf,
    excluded_dirs: Vec<String>,
}

impl ResourceManager {
    /// Getter for the vault root path
    pub fn vault_root(&self) -> &Path {
        self.vault_root.as_path()
    }

    /// Initializes the manager and canonicalizes the vault root to establish
    /// a firm security boundary against directory traversal.
    pub fn new<P: AsRef<Path>>(vault_root: P) -> Result<Self, io::Error> {
        let canonical_root = vault_root.as_ref().canonicalize()?;
        Ok(Self {
            vault_root: canonical_root,
            excluded_dirs: vec![
                ".obsidian".to_string(),
                ".git".to_string(),
                ".trash".to_string(),
            ],
        })
    }

    /// Handles `resources/list` endpoint
    pub fn list_resources(&self) -> Vec<Resource> {
        vec![Resource {
            uri: "vault://index".to_string(),
            name: "Vault Index".to_string(),
            mime_type: Some("text/plain".to_string()),
            description: Some(
                "A complete hierarchical list of all Markdown files in the Obsidian vault."
                    .to_string(),
            ),
        }]
    }

    /// Handles `resources/read` endpoint
    pub fn read_resource(&self, uri: &str) -> Result<ResourceContent, ResourceError> {
        match uri {
            "vault://index" => {
                let content = self.generate_vault_index()?;
                Ok(ResourceContent {
                    uri: uri.to_string(),
                    mime_type: Some("text/plain".to_string()),
                    text: content,
                })
            }
            _ => Err(ResourceError::NotFound(uri.to_string())),
        }
    }

    /// Recursively walks directory to build the markdown index
    fn generate_vault_index(&self) -> Result<String, ResourceError> {
        let mut index = String::from("# Obsidian Vault Index\n\n");
        let mut file_count = 0;

        for entry in WalkDir::new(&self.vault_root)
            .into_iter()
            .filter_entry(|e| self.is_not_excluded(e))
            .filter_map(Result::ok)
        // Filter out entries that resulted in an error or are excluded
        {
            if entry.file_type().is_file() {
                let path = entry.path();
                // Only include Markdown files
                if path.extension().is_some_and(|ext| ext == "md")
                    && let Ok(relative_path) = path.strip_prefix(&self.vault_root)
                {
                    // Normalize paths to forward slashes for cross-platform consistency
                    let display_path = relative_path.to_string_lossy().replace("\\", "/");
                    index.push_str(&format!("- {}\n", display_path));
                    file_count += 1;
                }
            }
        }

        if file_count == 0 {
            // If no markdown files were found, add a note to the index
            index.push_str("_No markdown files found in the vault._\n");
        }

        Ok(index)
    }

    /// Helper to skip ignored directory branches
    fn is_not_excluded(&self, entry: &DirEntry) -> bool {
        let file_name = entry.file_name().to_string_lossy(); // Convert entry name to string for comparison
        !self.excluded_dirs.iter().any(|ex| ex == &file_name) // Check if the entry is in the excluded list
    }
}
