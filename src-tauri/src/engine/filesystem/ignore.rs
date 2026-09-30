//! Ignore pattern matching engine
use std::path::Path;
use regex::Regex;
use ignore::gitignore::{Gitignore, GitignoreBuilder};

#[derive(Debug, Clone)]
pub struct IgnoreRules {
    gitignore: Option<Gitignore>,
    custom_patterns: Vec<CompiledPattern>,
    pub has_rules: bool,
}

#[derive(Debug, Clone)]
struct CompiledPattern {
    regex: Regex,
    is_allow: bool,
}

impl IgnoreRules {
    pub fn empty() -> Self {
        Self { gitignore: None, custom_patterns: Vec::new(), has_rules: false }
    }

    pub fn from_gitignore(gitignore_path: &Path) -> Self {
        let mut builder = GitignoreBuilder::new(gitignore_path.parent().unwrap_or(Path::new("")));
        if let Ok(content) = std::fs::read_to_string(gitignore_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') { continue; }
                builder.add_line(None, trimmed).ok();
            }
        }
        let gitignore = builder.build().ok();
        let has_rules = gitignore.is_some();
        Self { gitignore, custom_patterns: Vec::new(), has_rules }
    }

    pub fn from_content(content: &str, root: &Path) -> Self {
        let mut builder = GitignoreBuilder::new(root);
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') { continue; }
            builder.add_line(None, trimmed).ok();
        }
        let gitignore = builder.build().ok();
        let has_rules = gitignore.is_some();
        Self { gitignore, custom_patterns: Vec::new(), has_rules }
    }

    /// Set or replace the gitignore patterns from file content
    pub fn set_gitignore(&mut self, content: &str, root: &Path) {
        let mut builder = GitignoreBuilder::new(root);
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') { continue; }
            builder.add_line(None, trimmed).ok();
        }
        self.gitignore = builder.build().ok();
        if self.gitignore.is_some() {
            self.has_rules = true;
        }
    }

    pub fn add_custom_pattern(&mut self, pattern: &str) {
        let re_str = glob_to_regex(pattern);
        if let Ok(regex) = Regex::new(&re_str) {
            let is_allow = pattern.starts_with('!');
            self.custom_patterns.push(CompiledPattern { regex, is_allow });
            self.has_rules = true;
        }
    }

    pub fn add_custom_patterns(&mut self, patterns: &[String]) {
        for p in patterns { self.add_custom_pattern(p); }
    }

    pub fn is_ignored(&self, path: &Path, is_dir: bool) -> bool {
        let path_str = path.to_string_lossy().replace('\\', "/");
        for pattern in &self.custom_patterns {
            if pattern.regex.is_match(&path_str) {
                return !pattern.is_allow;
            }
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if pattern.regex.is_match(&name_str) {
                    return !pattern.is_allow;
                }
            }
        }
        if let Some(ref gitignore) = self.gitignore {
            match gitignore.matched(path, is_dir) {
                ignore::Match::Whitelist(_) => return false,
                ignore::Match::Ignore(_) => return true,
                ignore::Match::None => {}
            }
        }
        false
    }

    pub fn with_defaults() -> Self {
        let mut rules = Self::empty();
        let defaults = [
            ".git/", "node_modules/", ".venv/", "venv/", "__pycache__/",
            "*.pyc", "*.pyo", ".DS_Store", "Thumbs.db",
            "*.exe", "*.dll", "*.so", "*.dylib", "*.bin",
            "*.pdf", "*.png", "*.jpg", "*.jpeg", "*.gif", "*.ico",
            "*.svg", "*.woff", "*.woff2", "*.ttf", "*.eot",
            ".env", ".env.local", "vendor/", ".build/", "target/",
            "dist/", "build/", ".next/", ".nuxt/",
        ];
        for p in &defaults { rules.add_custom_pattern(p); }
        rules
    }
}

fn glob_to_regex(pattern: &str) -> String {
    let s = if pattern.starts_with('!') { &pattern[1..] } else { pattern };
    let mut re = String::from("(?i)");
    re.push('^');
    let match_basename = !s.contains('/') && !s.contains('\\');
    if s.starts_with('/') || s.starts_with('\\') {
        re.push_str(&s[1..]);
    } else if match_basename {
        re.push_str("(?:.*[/\\\\])?");
        // For basename-only patterns, we need to escape AFTER building the full string
        re.push_str(&escape_glob_chars(s));
    } else {
        // A slash makes this a path, not a basename. Match it in any directory.
        // Anchoring at the start left src/node_modules and an absolute path
        // from the scanner unmatched, so those files were included.
        re.push_str("(?:.*/)?");
        re.push_str(&escape_glob_chars(s));
    }
    if re.ends_with('/') { re.push_str(".*"); }
    re.push('$');
    re
}

/// Escape glob special characters (only in the pattern body, not the regex prefix)
fn escape_glob_chars(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '.' => escaped.push_str("\\."),
            '*' => escaped.push_str(".*"),
            '?' => escaped.push_str("."),
            '\\' => escaped.push_str("\\\\"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ignore() {
        let rules = IgnoreRules::with_defaults();
        assert!(rules.is_ignored(Path::new(".git/config"), false));
        assert!(rules.is_ignored(Path::new("node_modules/package.json"), false));
        assert!(!rules.is_ignored(Path::new("src/main.rs"), false));
    }

    #[test]
    fn test_custom_pattern() {
        let mut rules = IgnoreRules::empty();
        rules.add_custom_pattern("*.log");
        assert!(rules.is_ignored(Path::new("debug.log"), false));
        assert!(!rules.is_ignored(Path::new("main.rs"), false));
    }

    #[test]
    fn test_set_gitignore() {
        let mut rules = IgnoreRules::with_defaults();
        rules.set_gitignore("*.tmp\nbuild/", Path::new("/test"));
        let has_gitignore = rules.gitignore.is_some();
        assert!(has_gitignore, "gitignore should be set");
    }

    #[test]
    fn test_directory_pattern_matches_nested_and_absolute_paths() {
        let rules = IgnoreRules::with_defaults();
        assert!(rules.is_ignored(Path::new("src/node_modules/package.json"), false));
        assert!(rules.is_ignored(Path::new("/proj/node_modules/index.js"), false));
        assert!(!rules.is_ignored(Path::new("src/node_modules_backup/index.js"), false));
        assert!(!rules.is_ignored(Path::new("src/main.rs"), false));
    }
}

