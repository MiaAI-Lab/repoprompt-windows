//! MCP Server module for RepoPrompt
//!
//! Provides MCP (Model Context Protocol) tool definitions and a TCP server
//! so external AI agents can connect and use RepoPrompt capabilities.

use serde_json::{json, Value};

pub mod transport;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MCPToolResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

pub fn get_tools() -> Vec<MCPTool> {
    vec![
        MCPTool {
            name: "build_context".into(),
            description: "Build a context prompt from workspace files. Specify root directories and optional file filters.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "roots": { "type": "array", "items": {"type": "string"}, "description": "Root directories to scan" },
                    "selected_files": { "type": "array", "items": {"type": "string"}, "description": "Specific files to include (relative paths)" },
                    "token_budget": { "type": "integer", "description": "Maximum token count" },
                    "include_git_diff": { "type": "boolean", "description": "Include git diff" },
                    "user_instructions": { "type": "string", "description": "User instructions" }
                },
                "required": ["roots"]
            }),
        },
        MCPTool {
            name: "list_files".into(),
            description: "List files in a directory, respecting ignore rules.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Directory path" },
                    "include_hidden": { "type": "boolean", "description": "Include hidden files" }
                },
                "required": ["path"]
            }),
        },
        MCPTool {
            name: "read_file".into(),
            description: "Read the contents of a specific file.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "File path" },
                    "start_line": { "type": "integer", "description": "Starting line (1-based)" },
                    "end_line": { "type": "integer", "description": "Ending line" }
                },
                "required": ["path"]
            }),
        },
        MCPTool {
            name: "count_tokens".into(),
            description: "Count tokens in a text string.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string", "description": "Text to count" }
                },
                "required": ["text"]
            }),
        },
        MCPTool {
            name: "get_git_diff".into(),
            description: "Get git diff for a repository.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Repository path" }
                },
                "required": ["path"]
            }),
        },
        MCPTool {
            name: "get_repo_info".into(),
            description: "Get information about a git repository.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Repository path" }
                },
                "required": ["path"]
            }),
        },
    ]
}
