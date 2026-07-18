//! AI Optimization API domain.
//!
//! See <https://docs.dataforseo.com/v3/ai_optimization/overview/>.

mod ai_keyword_data;
mod llm_mentions;
mod llm_responses;
mod llm_scraper;
mod provider;

pub use ai_keyword_data::*;
pub use llm_mentions::*;
pub use llm_responses::*;
pub use llm_scraper::*;
pub use provider::*;

use crate::DataForSeoClient;

/// Entry point for the AI Optimization API.
pub struct AiOptimizationApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Returns the AI Optimization API builder.
    pub fn ai_optimization(&self) -> AiOptimizationApi<'_> {
        AiOptimizationApi { client: self }
    }
}

impl AiOptimizationApi<'_> {
    /// AI Keyword Data sub-API (AI search volume for keywords).
    pub fn ai_keyword_data(&self) -> AiOptimizationApiKeywordData<'_> {
        AiOptimizationApiKeywordData {
            client: self.client,
        }
    }

    /// LLM Mentions sub-API (brand/domain/keyword mentions in LLMs).
    pub fn llm_mentions(&self) -> AiOptimizationApiLlmMentions<'_> {
        AiOptimizationApiLlmMentions {
            client: self.client,
        }
    }

    /// ChatGPT provider (LLM Responses + LLM Scraper).
    pub fn chat_gpt(&self) -> AiOptimizationApiChatGpt<'_> {
        AiOptimizationApiChatGpt {
            client: self.client,
        }
    }

    /// Claude provider (LLM Responses).
    pub fn claude(&self) -> AiOptimizationApiClaude<'_> {
        AiOptimizationApiClaude {
            client: self.client,
        }
    }

    /// Gemini provider (LLM Responses + LLM Scraper).
    pub fn gemini(&self) -> AiOptimizationApiGemini<'_> {
        AiOptimizationApiGemini {
            client: self.client,
        }
    }

    /// Perplexity provider (LLM Responses, Live-only).
    pub fn perplexity(&self) -> AiOptimizationApiPerplexity<'_> {
        AiOptimizationApiPerplexity {
            client: self.client,
        }
    }
}
