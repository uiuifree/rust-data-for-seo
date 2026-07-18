//! Per-provider builders for the LLM Responses and LLM Scraper sub-APIs.

use super::{
    AiOptimizationApiLlmResponses, AiOptimizationApiLlmScraper,
    AiOptimizationApiPerplexityLlmResponses,
};
use crate::DataForSeoClient;

/// ChatGPT provider — exposes both LLM Responses and LLM Scraper.
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/overview/>.
pub struct AiOptimizationApiChatGpt<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl AiOptimizationApiChatGpt<'_> {
    /// LLM Responses endpoints for ChatGPT.
    pub fn llm_responses(&self) -> AiOptimizationApiLlmResponses<'_> {
        AiOptimizationApiLlmResponses {
            client: self.client,
            provider: "chat_gpt",
        }
    }

    /// LLM Scraper endpoints for ChatGPT.
    pub fn llm_scraper(&self) -> AiOptimizationApiLlmScraper<'_> {
        AiOptimizationApiLlmScraper {
            client: self.client,
            provider: "chat_gpt",
        }
    }
}

/// Claude provider — exposes LLM Responses only.
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/claude/llm_responses/overview/>.
pub struct AiOptimizationApiClaude<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl AiOptimizationApiClaude<'_> {
    /// LLM Responses endpoints for Claude.
    pub fn llm_responses(&self) -> AiOptimizationApiLlmResponses<'_> {
        AiOptimizationApiLlmResponses {
            client: self.client,
            provider: "claude",
        }
    }
}

/// Gemini provider — exposes both LLM Responses and LLM Scraper.
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/gemini/llm_responses/overview/>.
pub struct AiOptimizationApiGemini<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl AiOptimizationApiGemini<'_> {
    /// LLM Responses endpoints for Gemini.
    pub fn llm_responses(&self) -> AiOptimizationApiLlmResponses<'_> {
        AiOptimizationApiLlmResponses {
            client: self.client,
            provider: "gemini",
        }
    }

    /// LLM Scraper endpoints for Gemini.
    pub fn llm_scraper(&self) -> AiOptimizationApiLlmScraper<'_> {
        AiOptimizationApiLlmScraper {
            client: self.client,
            provider: "gemini",
        }
    }
}

/// Perplexity provider — exposes LLM Responses (Live-only).
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/perplexity/llm_responses/overview/>.
pub struct AiOptimizationApiPerplexity<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl AiOptimizationApiPerplexity<'_> {
    /// LLM Responses endpoints for Perplexity.
    pub fn llm_responses(&self) -> AiOptimizationApiPerplexityLlmResponses<'_> {
        AiOptimizationApiPerplexityLlmResponses {
            client: self.client,
        }
    }
}
