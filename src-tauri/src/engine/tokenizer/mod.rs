//! Token counting module using tiktoken-rs
//!
//! Provides token estimation for context budgeting using OpenAI's
//! tiktoken library (cl100k_base for GPT-4, p50k_base for GPT-3).

use std::collections::HashMap;
use tiktoken_rs::{p50k_base, cl100k_base, o200k_base};

/// Supported encoding types for token counting
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum EncodingType {
    /// cl100k_base - GPT-4, GPT-3.5-Turbo, GPT-4o
    Cl100kBase,
    /// p50k_base - GPT-3, Codex models
    P50kBase,
    /// o200k_base - o1, o3 models
    O200kBase,
}

impl EncodingType {
    pub fn name(&self) -> &'static str {
        match self {
            EncodingType::Cl100kBase => "cl100k_base",
            EncodingType::P50kBase => "p50k_base",
            EncodingType::O200kBase => "o200k_base",
        }
    }
}

/// Token counter with configurable encoding
#[derive(Debug, Clone)]
pub struct TokenCounter {
    /// Default encoding to use
    pub encoding: EncodingType,
    /// Cache of encoding instances
    #[allow(dead_code)]
    cache: HashMap<String, ()>,
}

impl TokenCounter {
    /// Create a new token counter with the specified encoding
    pub fn new(encoding: EncodingType) -> Self {
        Self {
            encoding,
            cache: HashMap::new(),
        }
    }

    /// Create with default cl100k_base encoding (GPT-4)
    pub fn default() -> Self {
        Self::new(EncodingType::Cl100kBase)
    }

    /// Count tokens in text using the configured encoding
    pub fn count_tokens(&self, text: &str) -> Result<usize, String> {
        match self.encoding {
            EncodingType::Cl100kBase => {
                let bpe = cl100k_base().map_err(|e| format!("Failed to init cl100k_base: {}", e))?;
                Ok(bpe.encode_with_special_tokens(text).len())
            }
            EncodingType::P50kBase => {
                let bpe = p50k_base().map_err(|e| format!("Failed to init p50k_base: {}", e))?;
                Ok(bpe.encode_with_special_tokens(text).len())
            }
            EncodingType::O200kBase => {
                let bpe = o200k_base().map_err(|e| format!("Failed to init o200k_base: {}", e))?;
                Ok(bpe.encode_with_special_tokens(text).len())
            }
        }
    }

    /// Count tokens with a specific encoding (overrides default)
    pub fn count_tokens_with(&self, text: &str, encoding: &EncodingType) -> Result<usize, String> {
        match encoding {
            EncodingType::Cl100kBase => {
                let bpe = cl100k_base().map_err(|e| format!("Failed to init cl100k_base: {}", e))?;
                Ok(bpe.encode_with_special_tokens(text).len())
            }
            EncodingType::P50kBase => {
                let bpe = p50k_base().map_err(|e| format!("Failed to init p50k_base: {}", e))?;
                Ok(bpe.encode_with_special_tokens(text).len())
            }
            EncodingType::O200kBase => {
                let bpe = o200k_base().map_err(|e| format!("Failed to init o200k_base: {}", e))?;
                Ok(bpe.encode_with_special_tokens(text).len())
            }
        }
    }

    /// Estimate tokens without loading the full encoding (fast approximate)
    pub fn estimate_tokens(text: &str) -> usize {
        // Rough estimate: ~4 characters per token on average for English text
        let char_count = text.chars().count();
        let word_count = text.split_whitespace().count();

        // Weighted average between char-based and word-based estimates
        let char_estimate = char_count / 4;
        let word_estimate = word_count * 3 / 2;

        std::cmp::max(char_estimate, word_estimate).max(1)
    }

    /// Get a human-readable label for the token count
    pub fn format_token_count(count: usize) -> String {
        if count >= 1_000_000 {
            format!("{:.1}M", count as f64 / 1_000_000.0)
        } else if count >= 1_000 {
            format!("{:.1}k", count as f64 / 1_000.0)
        } else {
            format!("{}", count)
        }
    }
}

/// Model-specific context window information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub context_window: usize,
    pub max_output_tokens: usize,
    pub encoding: EncodingType,
}

impl ModelInfo {
    /// Get info for well-known models
    pub fn known_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo { name: "gpt-4o".into(), context_window: 128_000, max_output_tokens: 16_384, encoding: EncodingType::O200kBase },
            ModelInfo { name: "gpt-4o-mini".into(), context_window: 128_000, max_output_tokens: 16_384, encoding: EncodingType::O200kBase },
            ModelInfo { name: "gpt-4-turbo".into(), context_window: 128_000, max_output_tokens: 4_096, encoding: EncodingType::Cl100kBase },
            ModelInfo { name: "gpt-4".into(), context_window: 8_192, max_output_tokens: 8_192, encoding: EncodingType::Cl100kBase },
            ModelInfo { name: "gpt-3.5-turbo".into(), context_window: 16_385, max_output_tokens: 4_096, encoding: EncodingType::Cl100kBase },
            ModelInfo { name: "claude-3-opus".into(), context_window: 200_000, max_output_tokens: 4_096, encoding: EncodingType::Cl100kBase },
            ModelInfo { name: "claude-3-sonnet".into(), context_window: 200_000, max_output_tokens: 4_096, encoding: EncodingType::Cl100kBase },
            ModelInfo { name: "claude-3-haiku".into(), context_window: 200_000, max_output_tokens: 4_096, encoding: EncodingType::Cl100kBase },
            ModelInfo { name: "claude-4-opus".into(), context_window: 200_000, max_output_tokens: 8_192, encoding: EncodingType::Cl100kBase },
            ModelInfo { name: "gpt-4.1".into(), context_window: 1_000_000, max_output_tokens: 32_768, encoding: EncodingType::O200kBase },
            ModelInfo { name: "o1".into(), context_window: 200_000, max_output_tokens: 100_000, encoding: EncodingType::O200kBase },
            ModelInfo { name: "o3-mini".into(), context_window: 200_000, max_output_tokens: 100_000, encoding: EncodingType::O200kBase },
            ModelInfo { name: "deepseek-chat".into(), context_window: 128_000, max_output_tokens: 8_192, encoding: EncodingType::Cl100kBase },
            ModelInfo { name: "gemini-2.0-flash".into(), context_window: 1_048_576, max_output_tokens: 8_192, encoding: EncodingType::Cl100kBase },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tokens() {
        let text = "The quick brown fox jumps over the lazy dog";
        let estimate = TokenCounter::estimate_tokens(text);
        assert!(estimate > 0);
    }

    #[test]
    fn test_counter() {
        let counter = TokenCounter::default();
        let count = counter.count_tokens("Hello, world! This is a test.").unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_format() {
        assert_eq!(TokenCounter::format_token_count(500), "500");
        assert_eq!(TokenCounter::format_token_count(1500), "1.5k");
        assert_eq!(TokenCounter::format_token_count(1500000), "1.5M");
    }

    #[test]
    fn test_known_models() {
        let models = ModelInfo::known_models();
        assert!(models.iter().any(|m| m.name.contains("gpt-4o")));
    }
}
