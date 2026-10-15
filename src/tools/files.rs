use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// --- Schemas for Tool Parameters ---

#[derive(Deserialize, JsonSchema)]
pub struct ListFilesParams {} // Empty struct to satisfy the Parameters extractor

#[derive(Deserialize, JsonSchema)]
pub struct ReadFileParams {
    pub uri: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ReadFilesParams {
    pub uris: Vec<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct CreateFileParams {
    #[schemars(description = "The target URI for the new note, starting with vault://")]
    pub uri: String,
    #[schemars(description = "The complete markdown content to write. DO NOT include a top-level `# Title` heading, as Obsidian automatically displays the filename as the H1.")]
    pub content: String,
}

// --- Helper Functions ---

fn resolve_uri(uri: &str, vault_root: &Path) -> Result<PathBuf, String> {
    let prefix = "vault://";
    if !uri.starts_with(prefix) {
        return Err("URI must start with vault://".to_string());
    }

    let relative_path = uri.strip_prefix(prefix).unwrap();
    let full_path = vault_root.join(relative_path);

    // Prevent directory traversal attacks (e.g., vault://../../../etc/passwd)
    if !full_path.starts_with(vault_root) {
        return Err("Path traversal blocked".to_string());
    }

    Ok(full_path)
}

// --- Tool Handlers ---

pub async fn list_files(_args: ListFilesParams, vault_root: &Path) -> Result<String, String> {
    let mut uris = Vec::new();

    for entry in WalkDir::new(vault_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "md")
            && let Ok(relative_path) = path.strip_prefix(vault_root)
        {
            let display_path = relative_path.to_string_lossy().replace("\\", "/");
            uris.push(format!("vault://{}", display_path));
        }
    }

    if uris.is_empty() {
        Ok("No markdown files found.".to_string())
    } else {
        Ok(uris.join("\n"))
    }
}

pub async fn read_file(args: ReadFileParams, vault_root: &Path) -> Result<String, String> {
    let path = resolve_uri(&args.uri, vault_root)?;

    fs::read_to_string(&path).map_err(|e| format!("Failed to read {}: {}", args.uri, e))
}

pub async fn read_files(args: ReadFilesParams, vault_root: &Path) -> Result<String, String> {
    let mut results = String::new();

    for uri in args.uris {
        results.push_str(&format!("--- Content of {} ---\n", uri));

        match resolve_uri(&uri, vault_root) {
            Ok(path) => match fs::read_to_string(&path) {
                Ok(content) => results.push_str(&content),
                Err(e) => results.push_str(&format!("Error reading file: {}\n", e)),
            },
            Err(e) => results.push_str(&format!("Error resolving URI: {}\n", e)),
        }
        results.push_str("\n\n");
    }

    Ok(results.trim_end().to_string())
}

pub async fn create_file(args: CreateFileParams, vault_root: &Path) -> Result<String, String> {
    // Resolve the URI to a safe local path using your existing helper
    let path = resolve_uri(&args.uri, vault_root)?;

    // Prevent accidental overwrites of existing notes
    if path.exists() {
        return Err(format!("File already exists at URI: {}", args.uri));
    }

    // As Obsidian allows nested directories, ensure parent directory exists before writing.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create parent directories for {}: {}",
                args.uri, e
            )
        })?;
    }

    // Write the content to the file
    fs::write(&path, args.content)
        .map_err(|e| format!("Failed to write file {}: {}", args.uri, e))?;

    Ok(format!("Successfully created {}", args.uri))
}
