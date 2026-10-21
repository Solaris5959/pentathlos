use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Deserialize, JsonSchema)]
pub struct FilenameSearchParams {
    #[schemars(
        description = "The partial or full target filename of the note to be read. Can be REGEX for more advanced searches."
    )]
    pub filename: String,
    #[schemars(
        description = "Optional. The relative vault sub-directory to search (e.g., 'Projects/'). If omitted, searches the entire vault."
    )]
    pub directory: Option<String>,
}

pub async fn filename_search(
    args: FilenameSearchParams,
    vault_root: &Path,
) -> Result<String, String> {
    let mut results = Vec::new();

    // Construct base directory to search
    let mut base_dir = vault_root.to_path_buf();
    if let Some(dir) = args.directory {
        // If subdirectory was included, check its validity
        let requested_path = Path::new(&dir);

        // Stop navigation outside vault
        if requested_path.is_absolute() || dir.contains("..") {
            return Err(
                "Invalid directory path. Path must be relative and within the vault.".to_string(),
            );
        }
        base_dir = base_dir.join(requested_path);
    }

    if !base_dir.exists() || !base_dir.is_dir() {
        return Err(format!(
            "The directory {} does not exist in the vault.",
            base_dir.display()
        ));
    }

    let filename_regex = Regex::new(&args.filename)
        .map_err(|e| format!("Failed to parse filename {}: {}", args.filename, e))?;

    for entry in WalkDir::new(base_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        // If filename matches, get its content, push the file path and content to results vector
        if filename_regex.is_match(&entry.file_name().to_string_lossy()) {
            let filepath = entry.into_path();

            let content = fs::read_to_string(&filepath)
                .map_err(|e| format!("Failed to read file {}: {}", filepath.display(), e))?;

            let formatted_output = format!("--- FILE: {} ---\n{}", filepath.display(), content);
            results.push(formatted_output);
        }
    }

    if results.is_empty() {
        Ok("No matching files found.".to_string())
    } else {
        Ok(results.join("\n"))
    }
}
