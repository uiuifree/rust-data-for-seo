//! Models for the LLM Scraper sub-API (ChatGPT, Gemini).
//!
//! See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/overview/>.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of the LLM Scraper `live/advanced` and `task_get/advanced` endpoints.
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/live/advanced/>.
///
/// The `items` array is heterogeneous (`chat_gpt_text`, `chat_gpt_table`,
/// `chat_gpt_products`, ...), so its elements are kept as raw JSON.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmScraper {
    /// Decoded keyword the scrape was run for.
    pub keyword: Option<String>,
    /// Location code the scrape was run for.
    pub location_code: Option<i32>,
    /// Language code the scrape was run for.
    pub language_code: Option<String>,
    /// Model version that produced the response (e.g. `"gpt-4o"`).
    pub model: Option<String>,
    /// Direct URL to the LLM results.
    pub check_url: Option<String>,
    /// UTC timestamp the scrape was performed.
    pub datetime: Option<String>,
    /// Full response content in markdown format.
    pub markdown: Option<String>,
    /// Web search results retrieved for the response (raw JSON).
    pub search_results: Option<Value>,
    /// Sources cited in the response (raw JSON).
    pub sources: Option<Value>,
    /// Related fan-out search queries (raw JSON).
    pub fan_out_queries: Option<Value>,
    /// Brands mentioned in the response (raw JSON).
    pub brand_entities: Option<Value>,
    /// Types of items present in [`items`](Self::items).
    pub item_types: Option<Vec<String>>,
    /// Number of items in [`items`](Self::items).
    pub items_count: Option<i64>,
    /// Parsed response items (heterogeneous, raw JSON per element).
    pub items: Option<Vec<Value>>,
}

/// A raw-HTML item of the LLM Scraper `*/html` endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmScraperHtmlItem {
    /// Item type.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Raw HTML content of the item.
    pub html: Option<String>,
}

/// Result of the LLM Scraper `live/html` and `task_get/html` endpoints.
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_scraper/live/html/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmScraperHtml {
    /// Decoded keyword the scrape was run for.
    pub keyword: Option<String>,
    /// Location code the scrape was run for.
    pub location_code: Option<i32>,
    /// Language code the scrape was run for.
    pub language_code: Option<String>,
    /// Model version that produced the response.
    pub model: Option<String>,
    /// Direct URL to the LLM results.
    pub check_url: Option<String>,
    /// UTC timestamp the scrape was performed.
    pub datetime: Option<String>,
    /// Number of items in [`items`](Self::items).
    pub items_count: Option<i64>,
    /// Raw-HTML response items.
    pub items: Option<Vec<AiOptimizationLlmScraperHtmlItem>>,
}
