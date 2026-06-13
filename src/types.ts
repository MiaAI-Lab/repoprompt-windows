// TypeScript types matching Rust structs

export interface FileEntry {
  path: string;
  relative_path: string;
  name: string;
  size: number;
  is_dir: boolean;
  extension: string;
}

export interface ScanStats {
  total_files: number;
  total_dirs: number;
  total_size: number;
  ignored_files: number;
  ignored_dirs: number;
  binary_files: number;
}

export interface LineRange {
  start: number;
  end: number;
  description: string | null;
}

export interface FileRangeRequest {
  file_path: string;
  ranges: LineRange[];
}

export interface ContextRequest {
  roots: string[];
  ignore_patterns: string[];
  token_budget: number;
  output_format: "Xml" | "Markdown" | "Text";
  include_file_tree: boolean;
  include_file_contents: boolean;
  include_git_diff: boolean;
  include_user_instructions: boolean;
  user_instructions: string;
  selected_files: string[];
  line_ranges: FileRangeRequest[];
  model_name: string | null;
}

export interface ContextResult {
  output: string;
  included_files: string[];
  token_count: number;
  char_count: number;
  file_tree: string | null;
  budget_truncated: boolean;
  remaining_budget: number;
  scan_stats: ScanStats | null;
  warnings: string[];
}

export interface RepoInfo {
  root: string;
  branch: string;
  has_uncommitted: boolean;
  has_untracked: boolean;
  is_detached: boolean;
}

export interface GitDiff {
  staged: string;
  unstaged: string;
  combined: string;
}

export interface ModelInfo {
  name: string;
  context_window: number;
  max_output_tokens: number;
  encoding: string;
}

export interface AppSettings {
  default_token_budget: number;
  model_name: string;
  encoding: string;
  custom_ignore_patterns: string[];
  recent_workspaces: string[];
  show_hidden_files: boolean;
  output_format: string;
  include_file_tree: boolean;
  include_git_diff: boolean;
  max_file_size: number;
}

export interface MCPTool {
  name: string;
  description: string;
  input_schema: any;
}

export type OutputFormat = "Xml" | "Markdown" | "Text";

export interface Workspace {
  path: string;
  name: string;
}

// Tree node for the file tree component
export interface TreeNode extends FileEntry {
  children: TreeNode[];
  checked: boolean;
  indeterminate: boolean;
  depth: number;
}

