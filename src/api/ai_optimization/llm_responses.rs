//! LLM Responses sub-API, shared by every provider.
//!
//! See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/overview/>.

use crate::entity::{
    AiOptimizationLlmResponse, AiOptimizationLlmResponseModel, AiOptimizationTaskReady,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

/// LLM Responses endpoints for a single provider (`chat_gpt`, `claude`, `gemini`).
///
/// These providers support both the Standard (`task_post`/`task_get`) and Live
/// methods. Perplexity is Live-only and exposed separately.
pub struct AiOptimizationApiLlmResponses<'a> {
    pub(crate) client: &'a DataForSeoClient,
    pub(crate) provider: &'static str,
}

impl AiOptimizationApiLlmResponses<'_> {
    /// Lists the models available for this provider.
    ///
    /// `GET /v3/ai_optimization/{provider}/llm_responses/models`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/models/>.
    pub async fn models(&self) -> DataForSeoApiResponse<AiOptimizationLlmResponseModel> {
        self.client
            .http_get(&format!(
                "/v3/ai_optimization/{}/llm_responses/models",
                self.provider
            ))
            .await
    }

    /// Generates a response synchronously.
    ///
    /// `POST /v3/ai_optimization/{provider}/llm_responses/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/live/>.
    pub async fn live(
        &self,
        data: Vec<AiOptimizationLlmResponsesPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmResponse> {
        self.client
            .http_post(
                &format!("/v3/ai_optimization/{}/llm_responses/live", self.provider),
                &data,
            )
            .await
    }

    /// Queues a response task (Standard method).
    ///
    /// `POST /v3/ai_optimization/{provider}/llm_responses/task_post`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/task_post/>.
    pub async fn task_post(
        &self,
        data: Vec<AiOptimizationLlmResponsesPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmResponse> {
        self.client
            .http_post(
                &format!(
                    "/v3/ai_optimization/{}/llm_responses/task_post",
                    self.provider
                ),
                &data,
            )
            .await
    }

    /// Lists tasks that have completed and are ready to collect.
    ///
    /// `GET /v3/ai_optimization/{provider}/llm_responses/tasks_ready`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/tasks_ready/>.
    pub async fn tasks_ready(&self) -> DataForSeoApiResponse<AiOptimizationTaskReady> {
        self.client
            .http_get(&format!(
                "/v3/ai_optimization/{}/llm_responses/tasks_ready",
                self.provider
            ))
            .await
    }

    /// Collects the result of a completed task by its `id`.
    ///
    /// `GET /v3/ai_optimization/{provider}/llm_responses/task_get/{id}`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/task_get/>.
    pub async fn task_get(&self, id: &str) -> DataForSeoApiResponse<AiOptimizationLlmResponse> {
        self.client
            .http_get(&format!(
                "/v3/ai_optimization/{}/llm_responses/task_get/{}",
                self.provider, id
            ))
            .await
    }
}

/// Perplexity LLM Responses endpoints (Live-only).
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/perplexity/llm_responses/overview/>.
pub struct AiOptimizationApiPerplexityLlmResponses<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl AiOptimizationApiPerplexityLlmResponses<'_> {
    /// Lists the Perplexity models available.
    ///
    /// `GET /v3/ai_optimization/perplexity/llm_responses/models`
    pub async fn models(&self) -> DataForSeoApiResponse<AiOptimizationLlmResponseModel> {
        self.client
            .http_get("/v3/ai_optimization/perplexity/llm_responses/models")
            .await
    }

    /// Generates a Perplexity response synchronously.
    ///
    /// `POST /v3/ai_optimization/perplexity/llm_responses/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/perplexity/llm_responses/live/>.
    pub async fn live(
        &self,
        data: Vec<AiOptimizationLlmResponsesPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmResponse> {
        self.client
            .http_post("/v3/ai_optimization/perplexity/llm_responses/live", &data)
            .await
    }
}

/// A single turn of a conversation passed via `message_chain`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMessage {
    /// Role of the message author (e.g. `"user"`, `"assistant"`).
    pub role: String,
    /// Text content of the message.
    pub message: String,
}

impl AiOptimizationLlmMessage {
    /// Creates a message with the given role and content.
    pub fn new(role: &str, message: &str) -> AiOptimizationLlmMessage {
        AiOptimizationLlmMessage {
            role: role.to_string(),
            message: message.to_string(),
        }
    }
}

/// Request body for the LLM Responses `live` and `task_post` endpoints.
///
/// `postback_url` and `pingback_url` apply only to `task_post`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmResponsesPost {
    /// The question or task to send to the model (max 500 characters).
    pub user_prompt: String,
    /// Model to use; a base name selects the latest version.
    pub model_name: String,
    /// Maximum tokens to generate (16–4096, default: 2048).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,
    /// Sampling temperature (0–2, default: 0.94; unsupported by reasoning models).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Nucleus sampling probability (0–1, default: 0.92; mutually exclusive with `temperature`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Whether the model may access the web (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search: Option<bool>,
    /// Whether to force the model to use web search (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_web_search: Option<bool>,
    /// ISO country code to localize web search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_country_iso_code: Option<String>,
    /// City name to localize web search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_city: Option<String>,
    /// System instructions steering the model (max 500 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_message: Option<String>,
    /// Prior conversation turns to include (max 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_chain: Option<Vec<AiOptimizationLlmMessage>>,
    /// URL to POST the result to when the task completes (`task_post` only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// URL to ping when the task completes (`task_post` only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl AiOptimizationLlmResponsesPost {
    /// Creates a request with the given prompt and model.
    pub fn new(user_prompt: &str, model_name: &str) -> AiOptimizationLlmResponsesPost {
        AiOptimizationLlmResponsesPost {
            user_prompt: user_prompt.to_string(),
            model_name: model_name.to_string(),
            ..Default::default()
        }
    }
}
