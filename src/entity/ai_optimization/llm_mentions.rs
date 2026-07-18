//! Models for the LLM Mentions sub-API.
//!
//! See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/overview/>.

use crate::entity::AiOptimizationMonthlySearch;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A grouped mention metric (`{ key, mentions, ai_search_volume }`).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationMentionMetric {
    /// Identifier of the group (e.g. a location, language or domain).
    pub key: Option<String>,
    /// Number of mentions within the group.
    pub mentions: Option<i64>,
    /// Aggregate AI search volume within the group.
    pub ai_search_volume: Option<f64>,
}

/// Aggregate totals for a group of mentions.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationMentionTotal {
    /// Total number of mentions.
    pub mentions: Option<i64>,
    /// Total AI search volume.
    pub ai_search_volume: Option<f64>,
}

/// Mention metrics grouped along every available dimension.
///
/// Shared by the aggregate and per-item breakdowns of the metrics and
/// top-mentioned endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationAggregatedMetrics {
    /// Mentions broken down by location.
    pub location: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by language.
    pub language: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by platform (`chat_gpt`, `google`).
    pub platform: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by domain cited as a source.
    pub sources_domain: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by domain appearing in search results.
    pub search_results_domain: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by brand-entity title.
    pub brand_entities_title: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by brand-entity category.
    pub brand_entities_category: Option<Vec<AiOptimizationMentionMetric>>,
    /// Totals across every group.
    pub total: Option<AiOptimizationMentionTotal>,
}

/// Source cited by an LLM answer inside a search-mentions result.
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/search/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationMentionSource {
    /// Text snippet extracted from the source.
    pub snippet: Option<String>,
    /// Display name of the source.
    pub source_name: Option<String>,
    /// URL of the source thumbnail image.
    pub thumbnail: Option<String>,
    /// Source content rendered as markdown.
    pub markdown: Option<String>,
    /// Position of the source within the answer.
    pub position: Option<i64>,
    /// Title of the source page.
    pub title: Option<String>,
    /// Domain of the source.
    pub domain: Option<String>,
    /// URL of the source.
    pub url: Option<String>,
    /// Publication date of the source, when available.
    pub publication_date: Option<String>,
}

/// A single mention returned by the `search/live` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMention {
    /// Platform the mention was found on (`chat_gpt`, `google`).
    pub platform: Option<String>,
    /// Name of the model that produced the answer.
    pub model_name: Option<String>,
    /// Location code the mention was recorded for.
    pub location_code: Option<i32>,
    /// Language code the mention was recorded for.
    pub language_code: Option<String>,
    /// The prompt/question that triggered the answer.
    pub question: Option<String>,
    /// The answer text (markdown format).
    pub answer: Option<String>,
    /// Sources cited by the answer.
    pub sources: Option<Vec<AiOptimizationMentionSource>>,
    /// Web search results attached to the answer (raw JSON).
    pub search_results: Option<Value>,
    /// AI search volume associated with the mention.
    pub ai_search_volume: Option<f64>,
    /// Monthly AI search-volume history for the mention.
    pub monthly_searches: Option<Vec<AiOptimizationMonthlySearch>>,
    /// UTC timestamp of the first recorded response.
    pub first_response_at: Option<String>,
    /// UTC timestamp of the most recent recorded response.
    pub last_response_at: Option<String>,
    /// Brand entities detected in the answer (raw JSON).
    pub brand_entities: Option<Value>,
    /// Fan-out queries derived from the prompt (raw JSON).
    pub fan_out_queries: Option<Value>,
}

/// Result of the `search/live` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMentionsSearch {
    /// Total number of mentions matching the request.
    pub total_count: Option<i64>,
    /// Offset of the first returned item.
    pub current_offset: Option<i32>,
    /// Token used to page beyond 20,000 results.
    pub search_after_token: Option<String>,
    /// Number of items in [`items`](Self::items).
    pub items_count: Option<i64>,
    /// The mentions returned for this page.
    pub items: Option<Vec<AiOptimizationLlmMention>>,
}

/// A per-item mention breakdown.
///
/// One item shape covers `target_metrics`, `multi_target_metrics` and every
/// `top_mentioned_*` endpoint; only the relevant identifier field
/// (`domain`, `page`, `brand`, `brand_category` or `key`) is populated.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationMentionItem {
    /// Grouping identifier (used by `multi_target_metrics`).
    pub key: Option<String>,
    /// Domain the item refers to (`top_mentioned_domains`).
    pub domain: Option<String>,
    /// Page URL the item refers to (`top_mentioned_pages`).
    pub page: Option<String>,
    /// Brand the item refers to (`top_mentioned_brands`).
    pub brand: Option<String>,
    /// Brand category the item refers to (`top_mentioned_brand_categories`).
    pub brand_category: Option<String>,
    /// Mentions broken down by location.
    pub location: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by language.
    pub language: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by platform.
    pub platform: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by domain cited as a source.
    pub sources_domain: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by domain appearing in search results.
    pub search_results_domain: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by brand-entity title.
    pub brand_entities_title: Option<Vec<AiOptimizationMentionMetric>>,
    /// Mentions broken down by brand-entity category.
    pub brand_entities_category: Option<Vec<AiOptimizationMentionMetric>>,
    /// Totals for this item.
    pub total: Option<AiOptimizationMentionTotal>,
}

/// Result shared by the metrics and top-mentioned endpoints.
///
/// Covers `target_metrics`, `multi_target_metrics`, `top_mentioned_pages`,
/// `top_mentioned_domains`, `top_mentioned_brands`,
/// `top_mentioned_brand_categories` and their `_lite` variants.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMentionsMetrics {
    /// Total number of items matching the request.
    pub total_count: Option<i64>,
    /// Offset of the first returned item.
    pub offset: Option<i32>,
    /// Offset of the first returned item (alternate field name).
    pub current_offset: Option<i32>,
    /// Number of items in [`items`](Self::items).
    pub items_count: Option<i64>,
    /// Metrics aggregated across every returned item.
    pub aggregated_metrics: Option<AiOptimizationAggregatedMetrics>,
    /// Per-item metric breakdowns.
    pub items: Option<Vec<AiOptimizationMentionItem>>,
}

/// A single point of a mentions timeseries.
///
/// Covers `historical`, `timeseries_delta` and `timeseries_new_lost`; only the
/// fields relevant to the requested endpoint are populated.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationMentionTimeseriesItem {
    /// Date of the data point (`yyyy-mm-dd`).
    pub date: Option<String>,
    /// Year of the data point.
    pub year: Option<i32>,
    /// Month of the data point (`1`–`12`).
    pub month: Option<i32>,
    /// Number of mentions in the period (`historical`).
    pub mentions: Option<i64>,
    /// AI search volume in the period (`historical`).
    pub ai_search_volume: Option<f64>,
    /// Mentions gained in the period (`timeseries_new_lost`).
    pub new_mentions: Option<i64>,
    /// Mentions lost in the period (`timeseries_new_lost`).
    pub lost_mentions: Option<i64>,
    /// AI search volume gained in the period (`timeseries_new_lost`).
    pub new_ai_search_volume: Option<f64>,
    /// AI search volume lost in the period (`timeseries_new_lost`).
    pub lost_ai_search_volume: Option<f64>,
    /// Mention count difference from the previous period (`timeseries_delta`).
    pub delta_mentions: Option<i64>,
    /// AI search volume difference from the previous period (`timeseries_delta`).
    pub delta_ai_search_volume: Option<f64>,
}

/// Result shared by the `historical`, `timeseries_delta` and
/// `timeseries_new_lost` endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMentionsTimeseries {
    /// Number of items in [`items`](Self::items).
    pub items_count: Option<i64>,
    /// The timeseries data points.
    pub items: Option<Vec<AiOptimizationMentionTimeseriesItem>>,
    /// Start date of the series (`yyyy-mm-dd`).
    pub date_from: Option<String>,
    /// End date of the series (`yyyy-mm-dd`).
    pub date_to: Option<String>,
    /// Grouping range of the series (`day`, `week`, `month`, `year`).
    pub group_range: Option<String>,
}
