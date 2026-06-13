//! RepoPrompt Engine - Core context engineering engine
//!
//! This crate provides the core functionality for RepoPrompt for Windows:
//! - File system scanning with .gitignore and custom ignore patterns
//! - Context building (file selection, line ranges, token budgeting)
//! - Prompt assembly in XML/TXT/MD formats
//! - Token counting for budget management
//! - Git integration (diff, repo detection)
//! - MCP server for external agent connectivity

pub mod context;
pub mod filesystem;
pub mod tokenizer;
pub mod git;
pub mod mcp;
pub mod settings;
