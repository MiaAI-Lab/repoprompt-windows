//! Context Builder — the core orchestrator for assembling context prompts

use std::path::{Path, PathBuf};
use crate::engine::filesystem::{FileEntry, ScanStats, LineRange};
use crate::engine::filesystem::scanner::{DirectoryScanner, ScanConfig};
use crate::engine::filesystem::content::{self, load_file, load_file_sliced, is_text_extension, is_binary_file};
use crate::engine::tokenizer::{TokenCounter, ModelInfo};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum OutputFormat {
    Xml,
    Markdown,
    Text,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextRequest {
    pub roots: Vec<PathBuf>,
    pub ignore_patterns: Vec<String>,
    pub token_budget: usize,
    pub output_format: OutputFormat,
    pub include_file_tree: bool,
    pub include_file_contents: bool,
    pub include_git_diff: bool,
    pub include_user_instructions: bool,
    pub user_instructions: String,
    pub selected_files: Vec<String>,
    pub line_ranges: Vec<FileRangeRequest>,
    pub model_name: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileRangeRequest {
    pub file_path: String,
    pub ranges: Vec<LineRange>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextResult {
    pub output: String,
    pub included_files: Vec<String>,
    pub token_count: usize,
    pub char_count: usize,
    pub file_tree: Option<String>,
    pub budget_truncated: bool,
    pub remaining_budget: usize,
    pub scan_stats: Option<ScanStats>,
    pub warnings: Vec<String>,
}

pub struct ContextBuilder {
    scanner: DirectoryScanner,
    token_counter: TokenCounter,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            scanner: DirectoryScanner::new(ScanConfig::default()),
            token_counter: TokenCounter::default(),
        }
    }

    pub fn with_config(config: ScanConfig, token_counter: TokenCounter) -> Self {
        Self {
            scanner: DirectoryScanner::new(config),
            token_counter,
        }
    }

    pub fn build(&self, request: ContextRequest) -> Result<ContextResult, String> {
        let mut warnings = Vec::new();
        let model = if let Some(ref model_name) = request.model_name {
            ModelInfo::known_models().into_iter().find(|m| m.name == *model_name)
        } else {
            None
        };

        let effective_budget = if request.token_budget > 0 {
            request.token_budget
        } else if let Some(ref m) = model {
            m.context_window - m.max_output_tokens
        } else {
            128_000
        };

        let mut all_files: Vec<FileEntry> = Vec::new();
        let mut scan_stats_total = ScanStats {
            total_files: 0, total_dirs: 0, total_size: 0,
            ignored_files: 0, ignored_dirs: 0, binary_files: 0,
        };

        for root in &request.roots {
            if !root.exists() {
                warnings.push(format!("Directory does not exist: {}", root.display()));
                continue;
            }

            let mut ignore_rules = DirectoryScanner::load_ignore_rules(root);
            ignore_rules.add_custom_patterns(&request.ignore_patterns);

            let (files, stats) = self.scanner.scan(root, &ignore_rules)?;

            let filtered: Vec<FileEntry> = if request.selected_files.is_empty() {
                files
            } else {
                files.into_iter().filter(|f| {
                    request.selected_files.iter().any(|s| f.relative_path.starts_with(s) || f.relative_path == *s)
                }).collect()
            };

            all_files.extend(filtered);
            scan_stats_total.total_files += stats.total_files;
            scan_stats_total.total_dirs += stats.total_dirs;
            scan_stats_total.total_size += stats.total_size;
            scan_stats_total.ignored_files += stats.ignored_files;
            scan_stats_total.ignored_dirs += stats.ignored_dirs;
        }

        let file_tree = if request.include_file_tree {
            Some(self.build_file_tree(&all_files, &request.roots))
        } else {
            None
        };

        let mut used_tokens = 0;
        let mut included_files: Vec<String> = Vec::new();
        let mut truncated = false;
        let mut output_parts: Vec<String> = Vec::new();

        if let Some(ref tree) = file_tree {
            used_tokens += self.token_counter.count_tokens(tree).unwrap_or(tree.len() / 4);
        }

        if request.include_file_contents {
            for file in &all_files {
                if file.is_dir || file.extension.is_empty() {
                    continue;
                }

                if is_binary_file(&file.path) {
                    continue;
                }

                if !is_text_extension(&file.extension) && content::is_binary_file(&file.path) {
                    continue;
                }

                let file_ranges: Vec<LineRange> = request.line_ranges.iter()
                    .find(|r| r.file_path == file.relative_path)
                    .map(|r| r.ranges.clone())
                    .unwrap_or_default();

                let file_content = if file_ranges.is_empty() {
                    load_file(file, None)?
                } else {
                    load_file_sliced(file, file_ranges, None)?
                };

                let block = format_file_block(&file.relative_path, &file_content.content);
                let block_tokens = self.token_counter.count_tokens(&block).unwrap_or(block.len() / 4);

                if used_tokens + block_tokens > effective_budget && used_tokens > 0 {
                    warnings.push(format!(
                        "Token budget exceeded ({}), stopped at {} tokens",
                        TokenCounter::format_token_count(effective_budget),
                        TokenCounter::format_token_count(used_tokens),
                    ));
                    truncated = true;
                    break;
                }

                output_parts.push(block);
                included_files.push(file.relative_path.clone());
                used_tokens += block_tokens;
            }
        }

        if request.include_git_diff {
            if let Some(root) = request.roots.first() {
                if let Ok(Some(diff)) = crate::engine::git::generate_diff(root) {
                    if !diff.combined.is_empty() {
                        let diff_block = format!("<git_diff>\n{}\n</git_diff>\n", diff.combined);
                        output_parts.push(diff_block);
                    }
                }
            }
        }

        let mut output = String::new();

        match request.output_format {
            OutputFormat::Xml => {
                output.push_str("<context>\n");
                if let Some(ref tree) = file_tree {
                    output.push_str(&format!("<file_map>\n{}</file_map>\n\n", tree));
                }
                if !output_parts.is_empty() {
                    output.push_str("<file_contents>\n");
                    for part in &output_parts {
                        output.push_str(part);
                        output.push('\n');
                    }
                    output.push_str("</file_contents>\n\n");
                }
                if !request.user_instructions.is_empty() {
                    output.push_str(&format!(
                        "<user_instructions>\n{}\n</user_instructions>\n",
                        request.user_instructions
                    ));
                }
                output.push_str("</context>");
            }
            OutputFormat::Markdown => {
                if let Some(ref tree) = file_tree {
                    output.push_str("# File Tree\n\n```\n");
                    output.push_str(tree);
                    output.push_str("```\n\n");
                }
                for part in &output_parts {
                    let first_line = part.lines().next().unwrap_or("");
                    output.push_str(&format!("## {}\n", first_line));
                    output.push_str(part);
                    output.push('\n');
                }
            }
            OutputFormat::Text => {
                if let Some(ref tree) = file_tree {
                    output.push_str("=== FILE TREE ===\n");
                    output.push_str(tree);
                    output.push_str("\n\n");
                }
                for part in &output_parts {
                    output.push_str(part);
                    output.push('\n');
                }
                if !request.user_instructions.is_empty() {
                    output.push_str("\n=== INSTRUCTIONS ===\n");
                    output.push_str(&request.user_instructions);
                    output.push('\n');
                }
            }
        }

        let char_count = output.chars().count();
        let final_token_count = self.token_counter.count_tokens(&output).unwrap_or(output.len() / 4);

        Ok(ContextResult {
            output,
            included_files,
            token_count: final_token_count,
            char_count,
            file_tree,
            budget_truncated: truncated,
            remaining_budget: if truncated { effective_budget.saturating_sub(used_tokens) } else { effective_budget.saturating_sub(final_token_count) },
            scan_stats: Some(scan_stats_total),
            warnings,
        })
    }

    fn build_file_tree(&self, files: &[FileEntry], roots: &[PathBuf]) -> String {
        let mut tree = String::new();

        for root in roots {
            let root_name = root.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| root.to_string_lossy().to_string());

            tree.push_str(&format!("📁 {}\n", root_name));

            let root_str = root.to_string_lossy().replace('\\', "/");
            let mut root_files: Vec<&FileEntry> = files.iter()
                .filter(|f| {
                    let f_path = f.path.to_string_lossy().replace('\\', "/");
                    f_path.starts_with(&root_str) || f.relative_path.starts_with(&root_name)
                })
                .collect();

            root_files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

            for file in &root_files {
                let parts: Vec<&str> = file.relative_path.split('/').collect();
                let indentation = "  ".repeat(parts.len().saturating_sub(1));
                tree.push_str(&format!("{}📄 {}\n", indentation, file.relative_path));
            }
        }

        tree
    }

    pub fn count_tokens(&self, text: &str) -> usize {
        self.token_counter.count_tokens(text).unwrap_or(text.len() / 4)
    }
}

pub fn format_file_block(path: &str, content: &str) -> String {
    let ext = Path::new(path).extension()
        .map(|e| e.to_string_lossy())
        .unwrap_or_else(|| std::borrow::Cow::Borrowed(""));
    format!("File: {}\n```{}\n{}\n```", path, ext, content.trim_end())
}

impl Default for ContextBuilder {
    fn default() -> Self { Self::new() }
}


