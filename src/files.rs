use std::path::PathBuf;
use walkdir::DirEntry;

#[derive(Debug)]
pub struct FileAndDestination {
    pub file_name: String,
    pub destination: PathBuf,
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
