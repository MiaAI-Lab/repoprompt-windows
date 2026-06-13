//! File content loading with line-range slicing support
//!
//! Reads file contents, handles encoding detection, and supports
//! extracting specific line ranges (slices) from files.

use std::path::Path;
use std::fs;
use std::io::Read;
use super::{FileEntry, FileContent, LineRange};

/// Detects whether a file is likely binary
pub fn is_binary_file(path: &Path) -> bool {
    if let Ok(mut file) = fs::File::open(path) {
        let mut buf = [0u8; 8192];
        if let Ok(n) = file.read(&mut buf) {
            // Check for null bytes which indicate binary content
            return buf[..n].contains(&0u8);
        }
    }
    false
}

/// Read file content with encoding detection
pub fn read_file_content(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }

    let bytes = fs::read(path)
        .map_err(|e| format!("Cannot read file {}: {}", path.display(), e))?;

    // Try UTF-8 first
    if let Ok(content) = String::from_utf8(bytes.clone()) {
        return Ok(content);
    }

    // Try other encodings
    // Use a simple heuristic: try to decode with lossy UTF-8
    let content = String::from_utf8_lossy(&bytes).to_string();
    Ok(content)
}

/// Read file content with line range slicing
pub fn read_file_content_sliced(
    path: &Path,
    ranges: &[LineRange],
) -> Result<String, String> {
    let content = read_file_content(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();

    let mut result = String::new();
    for range in ranges {
        let start = range.start.max(1).min(total_lines);
        let end = range.end.max(start).min(total_lines);

        if let Some(desc) = &range.description {
            result.push_str(&format!("# (lines {}-{}): {}\n", start, end, desc));
        }

        for i in start - 1..end {
            if i < lines.len() {
                result.push_str(lines[i]);
                result.push('\n');
            }
        }
        result.push('\n');
    }

    Ok(result.trim_end().to_string())
}

/// Get line count for a file
pub fn line_count(path: &Path) -> Result<usize, String> {
    let content = read_file_content(path)?;
    Ok(content.lines().count())
}

/// Read a whole file and return a FileContent
pub fn load_file(
    entry: &FileEntry,
    token_count: Option<usize>,
) -> Result<FileContent, String> {
    let content = read_file_content(&entry.path)?;
    let line_count = content.lines().count();
    let char_count = content.chars().count();

    Ok(FileContent {
        entry: entry.clone(),
        content,
        line_count,
        char_count,
        token_count,
        slices: None,
    })
}

/// Load a file with specific line ranges
pub fn load_file_sliced(
    entry: &FileEntry,
    ranges: Vec<LineRange>,
    token_count: Option<usize>,
) -> Result<FileContent, String> {
    let content = read_file_content_sliced(&entry.path, &ranges)?;
    let line_count = content.lines().count();
    let char_count = content.chars().count();

    Ok(FileContent {
        entry: entry.clone(),
        content,
        line_count,
        char_count,
        token_count,
        slices: Some(ranges),
    })
}

/// Check if a file extension is text (non-binary)
pub fn is_text_extension(ext: &str) -> bool {
    let text_extensions = [
        "rs", "go", "ts", "tsx", "js", "jsx", "py", "rb", "java", "kt", "swift",
        "c", "h", "cpp", "hpp", "cc", "cxx", "cs", "fs", "fsx",
        "html", "css", "scss", "sass", "less", "xml", "json", "yaml", "yml",
        "toml", "ini", "cfg", "conf", "md", "txt", "rst", "adoc",
        "sh", "bash", "zsh", "ps1", "bat", "cmd",
        "sql", "graphql", "proto", "gradle", "lock",
        "env", "editorconfig", "gitignore", "dockerfile",
        "vue", "svelte", "astro", "zig", "nim", "ex", "exs",
        "php", "pl", "pm", "lua", "r", "scala", "clj", "cljs",
        "kt", "kts", "dart", "m", "mm", "sqlite",
    ];
    text_extensions.contains(&ext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_read_content() {
        let dir = std::env::temp_dir().join("rp_test_read");
        fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test.txt");
        let mut f = fs::File::create(&file_path).unwrap();
        writeln!(f, "line1").unwrap();
        writeln!(f, "line2").unwrap();
        writeln!(f, "line3").unwrap();

        let content = read_file_content(&file_path).unwrap();
        assert!(content.contains("line1"));
        assert!(content.contains("line2"));
        assert!(content.contains("line3"));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_sliced_content() {
        let dir = std::env::temp_dir().join("rp_test_slice");
        fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test.txt");
        let mut f = fs::File::create(&file_path).unwrap();
        for i in 1..=10 {
            writeln!(f, "line{}", i).unwrap();
        }

        let ranges = vec![LineRange::new(2, 4)];
        let sliced = read_file_content_sliced(&file_path, &ranges).unwrap();
        assert!(sliced.contains("line2"));
        assert!(sliced.contains("line3"));
        assert!(sliced.contains("line4"));
        assert!(!sliced.contains("line1"));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_is_binary() {
        let dir = std::env::temp_dir().join("rp_test_bin");
        fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test.bin");
        let bytes = vec![0u8, 1, 2, 3, 255];
        fs::write(&file_path, &bytes).unwrap();
        assert!(is_binary_file(&file_path));
        fs::remove_dir_all(&dir).ok();
    }
}
