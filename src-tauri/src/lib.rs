mod engine;

use std::path::Path;
use tauri::State;
use std::sync::Mutex;

use engine::context::{ContextBuilder, ContextRequest, ContextResult};
use engine::filesystem::{FileEntry, ScanStats};
use engine::filesystem::scanner::{DirectoryScanner, ScanConfig};
use engine::filesystem::content::{read_file_content, read_file_content_sliced, is_binary_file};
use engine::tokenizer::{TokenCounter, EncodingType, ModelInfo};
use engine::git;
use engine::settings::AppSettings;
use engine::mcp;

pub struct AppState {
    pub context_builder: Mutex<ContextBuilder>,
    pub settings: Mutex<AppSettings>,
}

// ============================================================
// File System Commands
// ============================================================

#[tauri::command]
fn scan_directory(path: String, include_hidden: bool, max_depth: usize) -> Result<Vec<FileEntry>, String> {
    let root = Path::new(&path);
    if !root.exists() { return Err(format!("Path does not exist: {}", path)); }
    let ignore_rules = DirectoryScanner::load_ignore_rules(root);
    let config = ScanConfig { include_hidden, max_depth, ..Default::default() };
    let scanner = DirectoryScanner::new(config);
    let (files, _stats) = scanner.scan(root, &ignore_rules)?;
    Ok(files)
}

#[tauri::command]
fn scan_directory_with_stats(path: String, include_hidden: bool) -> Result<(Vec<FileEntry>, ScanStats), String> {
    let root = Path::new(&path);
    if !root.exists() { return Err(format!("Path does not exist: {}", path)); }
    let ignore_rules = DirectoryScanner::load_ignore_rules(root);
    let config = ScanConfig { include_hidden, ..Default::default() };
    let scanner = DirectoryScanner::new(config);
    scanner.scan(root, &ignore_rules)
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    let file_path = Path::new(&path);
    if !file_path.exists() { return Err(format!("File not found: {}", path)); }
    read_file_content(file_path)
}

#[tauri::command]
fn read_file_sliced(path: String, start_line: usize, end_line: usize) -> Result<String, String> {
    let file_path = Path::new(&path);
    if !file_path.exists() { return Err(format!("File not found: {}", path)); }
    let ranges = vec![engine::filesystem::LineRange::new(start_line, end_line)];
    read_file_content_sliced(file_path, &ranges)
}

#[tauri::command]
fn is_binary(path: String) -> bool {
    is_binary_file(Path::new(&path))
}

// ============================================================
// Context Building Commands
// ============================================================

#[tauri::command]
fn build_context(state: State<AppState>, request: ContextRequest) -> Result<ContextResult, String> {
    let builder = state.context_builder.lock().map_err(|e| e.to_string())?;
    builder.build(request)
}

#[tauri::command]
fn count_tokens(state: State<AppState>, text: String) -> Result<usize, String> {
    let builder = state.context_builder.lock().map_err(|e| e.to_string())?;
    Ok(builder.count_tokens(&text))
}

#[tauri::command]
fn count_tokens_with_model(text: String, model: String) -> Result<usize, String> {
    let encoding = ModelInfo::known_models().into_iter()
        .find(|m| m.name == model)
        .map(|m| m.encoding)
        .unwrap_or(EncodingType::Cl100kBase);
    let counter = TokenCounter::new(encoding);
    counter.count_tokens(&text)
}

// ============================================================
// Git Commands
// ============================================================

#[tauri::command]
fn check_git_repo(path: String) -> bool {
    git::is_git_repo(Path::new(&path))
}

#[tauri::command]
fn get_git_info(path: String) -> Result<Option<git::RepoInfo>, String> {
    git::get_repo_info(Path::new(&path))
}

#[tauri::command]
fn get_git_diff(path: String) -> Result<Option<git::GitDiff>, String> {
    git::generate_diff(Path::new(&path))
}

#[tauri::command]
fn get_git_diff_for_files(path: String, files: Vec<String>) -> Result<Option<git::GitDiff>, String> {
    git::generate_diff_for_files(Path::new(&path), &files)
}

#[tauri::command]
fn get_modified_files(path: String) -> Result<Vec<String>, String> {
    git::get_modified_files(Path::new(&path))
}

// ============================================================
// Settings Commands
// ============================================================

#[tauri::command]
fn get_settings(state: State<AppState>) -> Result<AppSettings, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(settings.clone())
}

#[tauri::command]
fn update_settings(state: State<AppState>, settings: AppSettings) -> Result<(), String> {
    let mut current = state.settings.lock().map_err(|e| e.to_string())?;
    *current = settings;
    Ok(())
}

#[tauri::command]
fn get_model_info() -> Vec<ModelInfo> {
    ModelInfo::known_models()
}

// ============================================================
// MCP Commands
// ============================================================

#[tauri::command]
fn get_mcp_tools() -> Vec<mcp::MCPTool> {
    mcp::get_tools()
}

#[tauri::command]
async fn start_mcp_server(port: u16) -> Result<(), String> {
    mcp::transport::start_server(port).await
}

#[tauri::command]
fn stop_mcp_server() {
    mcp::transport::stop_server();
}

#[tauri::command]
fn is_mcp_server_running() -> bool {
    mcp::transport::is_running()
}

// ============================================================
// Entry Point
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(AppState {
            context_builder: Mutex::new(ContextBuilder::new()),
            settings: Mutex::new(AppSettings::default()),
        })
        .invoke_handler(tauri::generate_handler![
            scan_directory, scan_directory_with_stats,
            read_file, read_file_sliced, is_binary,
            build_context, count_tokens, count_tokens_with_model,
            check_git_repo, get_git_info, get_git_diff, get_git_diff_for_files, get_modified_files,
            get_settings, update_settings, get_model_info, 
            get_mcp_tools, start_mcp_server, stop_mcp_server, is_mcp_server_running,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}






