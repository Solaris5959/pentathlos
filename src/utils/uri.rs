use std::path::{Path, PathBuf};

pub(crate) fn resolve_uri(uri: &str, vault_root: &Path) -> Result<PathBuf, String> {
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
