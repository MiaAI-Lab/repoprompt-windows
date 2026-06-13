use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepoInfo {
    pub root: PathBuf,
    pub branch: String,
    pub has_uncommitted: bool,
    pub has_untracked: bool,
    pub is_detached: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GitDiff {
    pub staged: String,
    pub unstaged: String,
    pub combined: String,
}

pub fn is_git_repo(path: &Path) -> bool {
    find_git_root(path).is_some()
}

pub fn find_git_root(path: &Path) -> Option<PathBuf> {
    let canonical = path.canonicalize().ok()?;
    let mut current = Some(canonical.as_path());
    while let Some(dir) = current {
        if dir.join(".git").exists() {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }
    None
}

pub fn get_repo_info(path: &Path) -> Result<Option<RepoInfo>, String> {
    let root = match find_git_root(path) {
        Some(r) => r,
        None => return Ok(None),
    };
    let branch = get_current_branch(&root);
    let (has_uncommitted, has_untracked) = check_status(&root);
    Ok(Some(RepoInfo { root, branch, has_uncommitted, has_untracked, is_detached: false }))
}

fn get_current_branch(repo_path: &Path) -> String {
    let path_str = repo_path.to_string_lossy();
    if let Ok(output) = Command::new("git")
        .args(["-C", &path_str, "rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        if output.status.success() {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if branch != "HEAD" { return branch; }
        }
    }
    if let Ok(repo) = git2::Repository::open(repo_path) {
        if let Ok(head) = repo.head() {
            if let Some(name) = head.shorthand() {
                return name.to_string();
            }
        }
    }
    "unknown".into()
}

fn check_status(repo_path: &Path) -> (bool, bool) {
    let mut has_uncommitted = false;
    let mut has_untracked = false;
    if let Ok(repo) = git2::Repository::open(repo_path) {
        if let Ok(statuses) = repo.statuses(None) {
            for entry in statuses.iter() {
                let status = entry.status();
                if status.contains(git2::Status::WT_NEW) { has_untracked = true; }
                if status.intersects(
                    git2::Status::WT_MODIFIED | git2::Status::WT_DELETED |
                    git2::Status::WT_RENAMED | git2::Status::WT_TYPECHANGE |
                    git2::Status::INDEX_NEW | git2::Status::INDEX_MODIFIED |
                    git2::Status::INDEX_DELETED
                ) { has_uncommitted = true; }
            }
        }
    }
    (has_uncommitted, has_untracked)
}

fn run_git_diff(repo_path: &Path, staged: bool, files: Option<&[String]>) -> Result<String, String> {
    let mut cmd = Command::new("git");
    let path_str = repo_path.to_string_lossy();
    cmd.arg("-C").arg(&*path_str).arg("diff");
    if staged { cmd.arg("--cached"); }
    cmd.arg("--no-color");
    if let Some(files) = files {
        for f in files { cmd.arg(f); }
    }
    let output = cmd.output().map_err(|e| format!("Failed to run git diff: {}", e))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("git diff failed: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

fn run_git_combined_diff(repo_path: &Path) -> Result<String, String> {
    let staged = run_git_diff(repo_path, true, None)?;
    let unstaged = run_git_diff(repo_path, false, None)?;
    let mut combined = String::new();
    if !staged.is_empty() { combined.push_str("# STAGED CHANGES\n"); combined.push_str(&staged); combined.push('\n'); }
    if !unstaged.is_empty() { combined.push_str("# UNSTAGED CHANGES\n"); combined.push_str(&unstaged); }
    Ok(combined)
}

fn run_git_diff_combined(repo_path: &Path, files: &[String]) -> Result<String, String> {
    let staged = run_git_diff(repo_path, true, Some(files))?;
    let unstaged = run_git_diff(repo_path, false, Some(files))?;
    let mut combined = String::new();
    if !staged.is_empty() { combined.push_str("# STAGED CHANGES\n"); combined.push_str(&staged); combined.push('\n'); }
    if !unstaged.is_empty() { combined.push_str("# UNSTAGED CHANGES\n"); combined.push_str(&unstaged); }
    Ok(combined)
}

pub fn generate_diff(repo_path: &Path) -> Result<Option<GitDiff>, String> {
    let root = match find_git_root(repo_path) { Some(r) => r, None => return Ok(None) };
    let staged = run_git_diff(&root, true, None)?;
    let unstaged = run_git_diff(&root, false, None)?;
    let combined = run_git_combined_diff(&root)?;
    Ok(Some(GitDiff { staged, unstaged, combined }))
}

pub fn generate_diff_for_files(repo_path: &Path, files: &[String]) -> Result<Option<GitDiff>, String> {
    let root = match find_git_root(repo_path) { Some(r) => r, None => return Ok(None) };
    let staged = run_git_diff(&root, true, Some(files))?;
    let unstaged = run_git_diff(&root, false, Some(files))?;
    let combined = run_git_diff_combined(&root, files)?;
    Ok(Some(GitDiff { staged, unstaged, combined }))
}

pub fn get_modified_files(repo_path: &Path) -> Result<Vec<String>, String> {
    let root = find_git_root(repo_path).ok_or_else(|| "Not a git repository".to_string())?;
    let mut files = Vec::new();
    if let Ok(repo) = git2::Repository::open(&root) {
        if let Ok(statuses) = repo.statuses(None) {
            for entry in statuses.iter() {
                let status = entry.status();
                if status != git2::Status::CURRENT {
                    if let Some(path) = entry.path() { files.push(path.to_string()); }
                }
            }
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}
