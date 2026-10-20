// ==========================================
// MODULES
// ==========================================
pub mod echo;
pub mod notes;
pub mod search;
pub mod vault;

// ==========================================
// IMPORTS
// ==========================================
// use crate::models::state::AppState;

// ==========================================
// FLATTENED IMPORTS
// ==========================================
pub use echo::{EchoParams, run};

// Vault structural tools
pub use vault::{ListFilesParams, list};

// Individual note lifecycle tools
pub use notes::{
    AppendFileParams, CreateFileParams, DeleteFileParams, EditFileParams, ReadFileParams,
    ReadFilesParams, append, create, delete, edit, read_file, read_files,
};
