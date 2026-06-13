//! Recursive directory scanner with ignore rules
//!
//! Walks directory trees respecting .gitignore, .repo_ignore, and custom
//! ignore patterns. Returns flat file listings suitable for UI file trees.

use std::path::Path;
use super::{FileEntry, ScanStats};
use super::ignore::IgnoreRules;

/// Scanner configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScanConfig {
    /// Whether to follow symlinks
    pub follow_symlinks: bool,
    /// Maximum directory depth (0 = unlimited)
    pub max_depth: usize,
    /// Whether to include hidden files/directories
    pub include_hidden: bool,
    /// Maximum file size in bytes to include (0 = unlimited)
    pub max_file_size: u64,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            follow_symlinks: false,
            max_depth: 0,
            include_hidden: false,
            max_file_size: 5 * 1024 * 1024, // 5MB default
        }
    }
}

/// Scans a directory tree and returns file entries respecting ignore rules.
pub struct DirectoryScanner {
    config: ScanConfig,
}

impl DirectoryScanner {
    pub fn new(config: ScanConfig) -> Self {
        Self { config }
    }

    /// Scan a directory root with the given ignore rules
    pub fn scan(
        &self,
        root: &Path,
        ignore_rules: &IgnoreRules,
    ) -> Result<(Vec<FileEntry>, ScanStats), String> {
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        let mut stats = ScanStats {
            total_files: 0,
            total_dirs: 0,
            total_size: 0,
            ignored_files: 0,
            ignored_dirs: 0,
            binary_files: 0,
        };

        if !root.exists() {
            return Err(format!("Directory does not exist: {}", root.display()));
        }
        if !root.is_dir() {
            return Err(format!("Path is not a directory: {}", root.display()));
        }

        let root_canonical = root.canonicalize().map_err(|e| format!("Cannot canonicalize path: {}", e))?;

        self.scan_recursive(
            &root_canonical,
            &root_canonical,
            0,
            ignore_rules,
            &mut files,
            &mut dirs,
            &mut stats,
        )?;

        Ok((files, stats))
    }

    fn scan_recursive(
        &self,
        current_dir: &Path,
        root: &Path,
        depth: usize,
        ignore_rules: &IgnoreRules,
        files: &mut Vec<FileEntry>,
        dirs: &mut Vec<FileEntry>,
        stats: &mut ScanStats,
    ) -> Result<(), String> {
        if self.config.max_depth > 0 && depth >= self.config.max_depth {
            return Ok(());
        }

        let read_dir = std::fs::read_dir(current_dir)
            .map_err(|e| format!("Cannot read directory {}: {}", current_dir.display(), e))?;

        for entry in read_dir {
            let entry = entry.map_err(|e| format!("Error reading entry: {}", e))?;
            let path = entry.path();

            // Skip hidden files/dirs if configured
            if !self.config.include_hidden {
                if let Some(name) = path.file_name() {
                    if name.to_string_lossy().starts_with('.') {
                        continue;
                    }
                }
            }

            let is_dir = path.is_dir();
            let is_symlink = path.is_symlink();

            // Skip symlinks if configured
            if is_symlink && !self.config.follow_symlinks {
                continue;
            }

            // Check ignore rules
            let relative = path.strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");

            if ignore_rules.has_rules && ignore_rules.is_ignored(&path, is_dir) {
                if is_dir {
                    stats.ignored_dirs += 1;
                } else {
                    stats.ignored_files += 1;
                }
                continue;
            }

            let metadata = std::fs::metadata(&path).ok();
            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
            let file_name = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let ext = path.extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            let file_entry = FileEntry {
                path: path.clone(),
                relative_path: relative.clone(),
                name: file_name,
                size,
                is_dir,
                extension: ext,
            };

            if is_dir {
                dirs.push(file_entry);
                stats.total_dirs += 1;
                // Recurse into subdirectories
                self.scan_recursive(
                    &path, root, depth + 1,
                    ignore_rules, files, dirs, stats,
                )?;
            } else {
                // Check file size limit
                if self.config.max_file_size > 0 && size > self.config.max_file_size {
                    stats.ignored_files += 1;
                    continue;
                }
                stats.total_files += 1;
                stats.total_size += size;
                files.push(file_entry);
            }
        }

        Ok(())
    }

    /// Quick scan - just get the file tree structure without reading content
    pub fn scan_tree(
        root: &Path,
        ignore_rules: &IgnoreRules,
    ) -> Result<Vec<FileEntry>, String> {
        let config = ScanConfig::default();
        let scanner = Self::new(config);
        let (files, _) = scanner.scan(root, ignore_rules)?;
        Ok(files)
    }

    /// Scan and load ignore rules from a root directory
    pub fn load_ignore_rules(root: &Path) -> IgnoreRules {
        let mut rules = IgnoreRules::with_defaults();

        // Load .gitignore
        let gitignore_path = root.join(".gitignore");
        if gitignore_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&gitignore_path) {
                rules.set_gitignore(&content, root);
                // Use gitignore crate's built-in pattern matching
            }
        }

        // Load .repo_ignore (custom to RepoPrompt)
        let repo_ignore_path = root.join(".repo_ignore");
        if repo_ignore_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&repo_ignore_path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() && !trimmed.starts_with('#') {
                        rules.add_custom_pattern(trimmed);
                    }
                }
            }
        }

        rules
    }
}




