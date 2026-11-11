use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub(crate) fn atomic_write(path: &Path, content: String) -> Result<(), std::io::Error> {
    // Create temporary filepath for atomic write
    let mut temp_path_str = path.as_os_str().to_os_string();
    temp_path_str.push("-temp");
    let temp_path = PathBuf::from(temp_path_str);

    // Open temporary file to write
    let mut temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp_path)?;

    // Write desired file content to temporary file
    if let Err(e) = temp_file.write_all(content.as_ref()) {
        // Remove the temporary file if this fails
        let _ = fs::remove_file(&temp_path);
        return Err(e);
    }

    // Force sync between filesystem and buffer
    if let Err(e) = temp_file.sync_all() {
        // Remove the temporary file if this fails
        let _ = fs::remove_file(&temp_path);
        return Err(e);
    }

    // Remove any references to or locks on temp file
    drop(temp_file);

    // Rename the newly created temporary file to the original file (overwrite it)
    if let Err(e) = fs::rename(&temp_path, path) {
        // Remove the temporary file if this fails
        let _ = fs::remove_file(&temp_path);
        return Err(e);
    }

    // Calculate the path to the parent directory
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let dir_path = if parent.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent
    };

    // Open the parent directory (should be platform agnostic) and sync file changes to disk
    if let Ok(dir) = File::open(dir_path) {
        let _ = dir.sync_all(); // Ignore errors here, as some OS/FS combos don't support directory syncing
    }

    Ok(())
}
