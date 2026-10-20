use schemars::JsonSchema;
use serde::Deserialize;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Deserialize, JsonSchema)]
pub struct ListFilesParams {} // Empty struct to satisfy the Parameters extractor

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
