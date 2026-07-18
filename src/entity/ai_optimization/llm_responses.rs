//! Models for the LLM Responses sub-API (ChatGPT, Claude, Gemini, Perplexity).
//!
//! See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/overview/>.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A model advertised by a provider's `models` endpoint.
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/models/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmResponseModel {
    /// Identifier of the model (e.g. `"gpt-4.1-mini"`).
    pub model_name: Option<String>,
    /// Whether the model is a reasoning model.
    pub reasoning: Option<bool>,
    /// Whether the model supports web search.
    pub web_search_supported: Option<bool>,
    /// Whether the model supports the Standard (`task_post`) method.
    pub task_post_supported: Option<bool>,
}

/// A source citation attached to a response section.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmResponseAnnotation {
    /// Title of the cited source.
    pub title: Option<String>,
    /// URL of the cited source.
    pub url: Option<String>,
}

/// A content section inside a response item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmResponseSection {
    /// Section type (`"text"`, `"summary_text"`, ...).
    #[serde(rename = "type")]
    pub section_type: Option<String>,
    /// Text content of the section.
    pub text: Option<String>,
    /// Source citations referenced by the section.
    pub annotations: Option<Vec<AiOptimizationLlmResponseAnnotation>>,
}

/// An item of an LLM response (either a `reasoning` or a `message` block).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmResponseItem {
    /// Item type (`"reasoning"` or `"message"`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Content sections of the item.
    pub sections: Option<Vec<AiOptimizationLlmResponseSection>>,
}

/// Result of the LLM Responses `live`, `task_post` and `task_get` endpoints.
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmResponse {
    /// Actual model used to generate the response.
    pub model_name: Option<String>,
    /// Number of input (prompt) tokens.
    pub input_tokens: Option<i64>,
    /// Number of output (completion) tokens.
    pub output_tokens: Option<i64>,
    /// Number of reasoning tokens (reasoning models only).
    pub reasoning_tokens: Option<i64>,
    /// Whether web search was used to produce the response.
    pub web_search: Option<bool>,
    /// Third-party AI provider cost in USD.
    pub money_spent: Option<f64>,
    /// UTC timestamp the response was generated.
    pub datetime: Option<String>,
    /// Response content items (reasoning and message blocks).
    pub items: Option<Vec<AiOptimizationLlmResponseItem>>,
    /// Related fan-out search queries (raw JSON).
    pub fan_out_queries: Option<Value>,
}
