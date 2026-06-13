//! Settings and configuration management

use crate::engine::tokenizer::{ModelInfo};

/// Application-wide settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
    /// Default token budget for context building
    pub default_token_budget: usize,
    /// Model context limit
    pub model_name: String,
    /// Default encoding for token counting
    pub encoding: String,
    /// Custom ignore patterns
    pub custom_ignore_patterns: Vec<String>,
    /// Recent workspace paths
    pub recent_workspaces: Vec<String>,
    /// Whether to show hidden files by default
    pub show_hidden_files: bool,
    /// Default output format
    pub output_format: String,
    /// Include file tree by default
    pub include_file_tree: bool,
    /// Include git diff by default
    pub include_git_diff: bool,
    /// Max file size (bytes) to include
    pub max_file_size: u64,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            default_token_budget: 128_000,
            model_name: "gpt-4o".into(),
            encoding: "cl100k_base".into(),
            custom_ignore_patterns: vec![
                ".git/".into(),
                "node_modules/".into(),
                ".venv/".into(),
                "__pycache__/".into(),
                "*.pyc".into(),
                ".DS_Store".into(),
                "Thumbs.db".into(),
                "target/".into(),
                "dist/".into(),
                ".next/".into(),
            ],
            recent_workspaces: Vec::new(),
            show_hidden_files: false,
            output_format: "xml".into(),
            include_file_tree: true,
            include_git_diff: false,
            max_file_size: 5 * 1024 * 1024,
        }
    }
}

impl AppSettings {
    /// Get model info for the currently selected model
    pub fn model_info(&self) -> Option<ModelInfo> {
        ModelInfo::known_models().into_iter().find(|m| m.name == self.model_name)
    }

    /// Get token budget (from settings or model default)
    pub fn effective_budget(&self) -> usize {
        if self.default_token_budget > 0 {
            self.default_token_budget
        } else if let Some(model) = self.model_info() {
            model.context_window
        } else {
            128_000
        }
    }
}

