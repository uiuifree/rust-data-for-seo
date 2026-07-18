//! LLM Scraper sub-API (ChatGPT, Gemini).
//!
//! See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/overview/>.

use crate::entity::{
    AiOptimizationLanguage, AiOptimizationLlmScraper, AiOptimizationLlmScraperHtml,
    AiOptimizationScraperLocation, AiOptimizationTaskReady,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

/// LLM Scraper endpoints for a single provider (`chat_gpt`, `gemini`).
pub struct AiOptimizationApiLlmScraper<'a> {
    pub(crate) client: &'a DataForSeoClient,
    pub(crate) provider: &'static str,
}

impl AiOptimizationApiLlmScraper<'_> {
    /// Lists the locations supported by this provider's scraper.
    ///
    /// `GET /v3/ai_optimization/{provider}/llm_scraper/locations`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/locations/>.
    pub async fn locations(&self) -> DataForSeoApiResponse<AiOptimizationScraperLocation> {
        self.client
            .http_get(&format!(
                "/v3/ai_optimization/{}/llm_scraper/locations",
                self.provider
            ))
            .await
    }

    /// Lists the languages supported by this provider's scraper.
    ///
    /// `GET /v3/ai_optimization/{provider}/llm_scraper/languages`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/languages/>.
    pub async fn languages(&self) -> DataForSeoApiResponse<AiOptimizationLanguage> {
        self.client
            .http_get(&format!(
                "/v3/ai_optimization/{}/llm_scraper/languages",
                self.provider
            ))
            .await
    }

    /// Queues a scraper task (Standard method).
    ///
    /// `POST /v3/ai_optimization/{provider}/llm_scraper/task_post`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/task_post/>.
    pub async fn task_post(
        &self,
        data: Vec<AiOptimizationLlmScraperPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmScraper> {
        self.client
            .http_post(
                &format!(
                    "/v3/ai_optimization/{}/llm_scraper/task_post",
                    self.provider
                ),
                &data,
            )
            .await
    }

    /// Lists scraper tasks that have completed and are ready to collect.
    ///
    /// `GET /v3/ai_optimization/{provider}/llm_scraper/tasks_ready`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/tasks_ready/>.
    pub async fn tasks_ready(&self) -> DataForSeoApiResponse<AiOptimizationTaskReady> {
        self.client
            .http_get(&format!(
                "/v3/ai_optimization/{}/llm_scraper/tasks_ready",
                self.provider
            ))
            .await
    }

    /// Collects a completed task's parsed result by its `id`.
    ///
    /// `GET /v3/ai_optimization/{provider}/llm_scraper/task_get/advanced/{id}`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/task_get/advanced/>.
    pub async fn task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<AiOptimizationLlmScraper> {
        self.client
            .http_get(&format!(
                "/v3/ai_optimization/{}/llm_scraper/task_get/advanced/{}",
                self.provider, id
            ))
            .await
    }

    /// Collects a completed task's raw HTML by its `id`.
    ///
    /// `GET /v3/ai_optimization/{provider}/llm_scraper/task_get/html/{id}`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/task_get/html/>.
    pub async fn task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<AiOptimizationLlmScraperHtml> {
        self.client
            .http_get(&format!(
                "/v3/ai_optimization/{}/llm_scraper/task_get/html/{}",
                self.provider, id
            ))
            .await
    }

    /// Scrapes and parses a result synchronously.
    ///
    /// `POST /v3/ai_optimization/{provider}/llm_scraper/live/advanced`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/live/advanced/>.
    pub async fn live_advanced(
        &self,
        data: Vec<AiOptimizationLlmScraperPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmScraper> {
        self.client
            .http_post(
                &format!(
                    "/v3/ai_optimization/{}/llm_scraper/live/advanced",
                    self.provider
                ),
                &data,
            )
            .await
    }

    /// Scrapes and returns the raw HTML synchronously.
    ///
    /// `POST /v3/ai_optimization/{provider}/llm_scraper/live/html`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/live/html/>.
    pub async fn live_html(
        &self,
        data: Vec<AiOptimizationLlmScraperPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmScraperHtml> {
        self.client
            .http_post(
                &format!(
                    "/v3/ai_optimization/{}/llm_scraper/live/html",
                    self.provider
                ),
                &data,
            )
            .await
    }
}

/// Request body for the LLM Scraper `live/*` and `task_post` endpoints.
///
/// `postback_url` and `pingback_url` apply only to `task_post`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmScraperPost {
    /// Keyword to scrape (max 2000 characters).
    pub keyword: String,
    /// Full name of the search location (alternative to `location_code`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Search location code (alternative to `location_name`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full name of the search language (alternative to `language_code`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Search language code (alternative to `language_name`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether to force the model to access and cite current web information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_web_search: Option<bool>,
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

impl AiOptimizationLlmScraperPost {
    /// Creates a request for the given keyword.
    pub fn new(keyword: &str) -> AiOptimizationLlmScraperPost {
        AiOptimizationLlmScraperPost {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}
