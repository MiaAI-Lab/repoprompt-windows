//! File system scanning module with ignore pattern support

use std::path::PathBuf;

pub mod scanner;
pub mod ignore;
pub mod content;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub relative_path: String,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub extension: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScanStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_size: u64,
    pub ignored_files: usize,
    pub ignored_dirs: usize,
    pub binary_files: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LineRange {
    pub start: usize,
    pub end: usize,
    pub description: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileContent {
    pub entry: FileEntry,
    pub content: String,
    pub line_count: usize,
    pub char_count: usize,
    pub token_count: Option<usize>,
    pub slices: Option<Vec<LineRange>>,
}

impl LineRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end, description: None }
    }

    pub fn with_description(start: usize, end: usize, description: String) -> Self {
        Self { start, end, description: Some(description) }
    }
}
