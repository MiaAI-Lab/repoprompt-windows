import { invoke } from "@tauri-apps/api/core";
import type {
  FileEntry,
  ScanStats,
  ContextRequest,
  ContextResult,
  RepoInfo,
  GitDiff,
  ModelInfo,
  AppSettings,
  MCPTool,
} from "../types";

// ---- File System ----

export async function scanDirectory(
  path: string,
  includeHidden = false,
  maxDepth = 0
): Promise<FileEntry[]> {
  return invoke("scan_directory", { path, includeHidden, maxDepth });
}

export async function scanDirectoryWithStats(
  path: string,
  includeHidden = false
): Promise<[FileEntry[], ScanStats]> {
  return invoke("scan_directory_with_stats", { path, includeHidden });
}

export async function readFile(path: string): Promise<string> {
  return invoke("read_file", { path });
}

export async function readFileSliced(
  path: string,
  startLine: number,
  endLine: number
): Promise<string> {
  return invoke("read_file_sliced", { path, startLine, endLine });
}

export async function isBinary(path: string): Promise<boolean> {
  return invoke("is_binary", { path });
}

// ---- Context Building ----

export async function buildContext(
  request: ContextRequest
): Promise<ContextResult> {
  return invoke("build_context", { request });
}

export async function countTokens(text: string): Promise<number> {
  return invoke("count_tokens", { text });
}

export async function countTokensWithModel(
  text: string,
  model: string
): Promise<number> {
  return invoke("count_tokens_with_model", { text, model });
}

// ---- Token Counting ----

export async function countTokensForFile(
  path: string
): Promise<[number, number, number]> {
  const content = await readFile(path);
  const tokens = await countTokens(content);
  const lines = content.split("\n").length;
  const chars = content.length;
  return [tokens, chars, lines];
}

// ---- Git Integration ----

export async function checkGitRepo(path: string): Promise<boolean> {
  return invoke("check_git_repo", { path });
}

export async function getGitInfo(path: string): Promise<RepoInfo | null> {
  return invoke("get_git_info", { path });
}

export async function getGitDiff(path: string): Promise<GitDiff | null> {
  return invoke("get_git_diff", { path });
}

export async function getGitDiffForFiles(
  path: string,
  files: string[]
): Promise<GitDiff | null> {
  return invoke("get_git_diff_for_files", { path, files });
}

export async function getModifiedFiles(
  path: string
): Promise<string[]> {
  return invoke("get_modified_files", { path });
}

// ---- Settings ----

export async function getSettings(): Promise<AppSettings> {
  return invoke("get_settings");
}

export async function updateSettings(
  settings: AppSettings
): Promise<void> {
  return invoke("update_settings", { settings });
}

export async function getModelInfo(): Promise<ModelInfo[]> {
  return invoke("get_model_info");
}

// ---- MCP ----

export async function getMcpTools(): Promise<MCPTool[]> {
  return invoke("get_mcp_tools");
}

// ---- Utility ----

export async function getTokenEstimate(text: string): Promise<number> {
  return invoke("get_token_estimate", { text });
}

export async function formatFileBlock(
  path: string,
  content: string
): Promise<string> {
  return invoke("format_file_block", { path, content });
}
