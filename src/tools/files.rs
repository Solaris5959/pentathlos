use schemars::JsonSchema;
use serde::Deserialize;
use std::fs::{self};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// --- Schemas for Tool Parameters ---

#[derive(Deserialize, JsonSchema)]
pub struct ListFilesParams {} // Empty struct to satisfy the Parameters extractor

#[derive(Deserialize, JsonSchema)]
pub struct ReadFileParams {
    #[schemars(description = "The target URI of the note to be read, starting with vault:// ")]
    pub uri: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ReadFilesParams {
    #[schemars(
        description = "The target list of URIs of the notes to be read, starting with vault:// "
    )]
    pub uris: Vec<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct CreateFileParams {
    #[schemars(description = "The target URI for the new note, starting with vault://")]
    pub uri: String,

    #[schemars(
        description = "The complete markdown content to write. DO NOT include a top-level `# Title` heading, as Obsidian automatically displays the filename as the H1."
    )]
    pub content: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct DeleteFileParams {
    #[schemars(description = "The target URI for the note to be deleted, starting with vault://")]
    pub uri: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct AppendFileParams {
    #[schemars(
        description = "The target URI for the note to be appended to, starting with vault://"
    )]
    pub uri: String,

    #[schemars(
        description = "The additional markdown content to be appended to the existing file contents."
    )]
    pub new_content: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct EditFileParams {
    #[schemars(description = "The target URI for the note to be modified, starting with vault://")]
    pub uri: String,

    #[schemars(
        description = "The exact markdown content to insert in place of the removed lines. The number of lines inserted does not need to match the number of lines removed."
    )]
    pub new_content: String,

    #[schemars(description = "The 1-based line number where the replacement should begin.")]
    pub start_line: u32,

    #[schemars(
        description = "The 1-based line number where the replacement should end (inclusive)."
    )]
    pub end_line: u32,
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

pub async fn list(_args: ListFilesParams, vault_root: &Path) -> Result<String, String> {
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

pub async fn create(args: CreateFileParams, vault_root: &Path) -> Result<String, String> {
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

pub async fn delete(args: DeleteFileParams, vault_root: &Path) -> Result<String, String> {
    let path = resolve_uri(&args.uri, vault_root)?;

    if path.exists() {
        fs::remove_file(path).map_err(|e| format!("Failed to delete file {}: {}", args.uri, e))?;

        Ok(format!("Successfully deleted {}", args.uri))
    } else {
        Err(format!("File does not exists at URI: {}", args.uri))
    }
}

pub async fn append(args: AppendFileParams, vault_root: &Path) -> Result<String, String> {
    let path = resolve_uri(&args.uri, vault_root)?;

    if let Ok(mut file_content) = fs::read_to_string(&path) {
        file_content.push('\n');
        file_content.push_str(&args.new_content);

        fs::write(path, file_content)
            .map_err(|e| format!("Failed to delete file {}: {}", args.uri, e))?;

        Ok(format!("Successfully appended content to {}", args.uri))
    } else {
        Err(format!("File does not exists at URI: {}", args.uri))
    }
}

pub async fn edit(args: EditFileParams, vault_root: &Path) -> Result<String, String> {
    let path = resolve_uri(&args.uri, vault_root)?;

    // Read current file content
    let file_content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file {}: {}", args.uri, e))?;

    // Split file contents into a vector of lines
    let mut lines: Vec<&str> = file_content.lines().collect();

    // Split new content into vector of lines
    let new_lines: Vec<&str> = args.new_content.lines().collect();

    // Line start and end numbers for the file segment to be replaced
    let start_idx = args.start_line.saturating_sub(1) as usize;
    let mut end_idx = args.end_line as usize; // Don't decrement end, as splice is exclusive at upper bound
    if end_idx > lines.len() {
        // Clamp end index to end of file
        end_idx = lines.len();
    }

    // Check bounds validity
    if start_idx > lines.len() || end_idx > lines.len() || start_idx >= end_idx {
        return Err(format!(
            "Bounds error: requested {}-{}, but file only has {} lines.",
            args.start_line,
            args.end_line,
            lines.len()
        ));
    }

    // Replace removed line range with new lines, number of lines does not necessarily match
    lines.splice(start_idx..end_idx, new_lines);

    // Join vector to create the updated file content
    let mut updated_content = lines.join("\n");
    if !updated_content.ends_with('\n') {
        // ensure updated file ends in newline
        updated_content.push('\n');
    }

    // Write to file with updated contents
    fs::write(&path, updated_content).map_err(|e| format!("Failed to update file: {}", e))?;

    Ok(format!(
        "Successfully replaced lines {}-{} in {}.\n\nWARNING/NOTE: The file now contains {} total lines. If you left orphaned text at the bottom, use the edit tool again to remove it.",
        args.start_line,
        args.end_line,
        args.uri,
        lines.len()
    ))
}
