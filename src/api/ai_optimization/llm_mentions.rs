//! LLM Mentions sub-API.
//!
//! Every endpoint is Live-only and delivers instant results.
//! See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/overview/>.

use crate::entity::{
    AiOptimizationLlmMentionsMetrics, AiOptimizationLlmMentionsSearch,
    AiOptimizationLlmMentionsTimeseries, AiOptimizationLocationLanguage,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

/// Endpoints under `/v3/ai_optimization/llm_mentions/`.
pub struct AiOptimizationApiLlmMentions<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl AiOptimizationApiLlmMentions<'_> {
    /// Lists the locations and languages supported by LLM Mentions.
    ///
    /// `GET /v3/ai_optimization/llm_mentions/locations_and_languages`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/locations_and_languages/>.
    pub async fn locations_and_languages(
        &self,
    ) -> DataForSeoApiResponse<AiOptimizationLocationLanguage> {
        self.client
            .http_get("/v3/ai_optimization/llm_mentions/locations_and_languages")
            .await
    }

    /// Returns individual mentions matching the targets.
    ///
    /// `POST /v3/ai_optimization/llm_mentions/search/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/search/live/>.
    pub async fn search(
        &self,
        data: Vec<AiOptimizationLlmMentionsSearchPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsSearch> {
        self.client
            .http_post("/v3/ai_optimization/llm_mentions/search/live", &data)
            .await
    }

    /// Returns aggregated mention metrics for the targets.
    ///
    /// `POST /v3/ai_optimization/llm_mentions/target_metrics/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/target_metrics/live/>.
    pub async fn target_metrics(
        &self,
        data: Vec<AiOptimizationLlmMentionsMetricsPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/target_metrics/live",
                &data,
            )
            .await
    }

    /// Lighter, lower-cost variant of [`target_metrics`](Self::target_metrics).
    ///
    /// `POST /v3/ai_optimization/llm_mentions/target_metrics_lite/live`
    pub async fn target_metrics_lite(
        &self,
        data: Vec<AiOptimizationLlmMentionsMetricsPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/target_metrics_lite/live",
                &data,
            )
            .await
    }

    /// Returns aggregated mention metrics grouped per named target.
    ///
    /// `POST /v3/ai_optimization/llm_mentions/multi_target_metrics/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/multi_target_metrics/live/>.
    pub async fn multi_target_metrics(
        &self,
        data: Vec<AiOptimizationLlmMentionsMultiTargetMetricsPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/multi_target_metrics/live",
                &data,
            )
            .await
    }

    /// Returns the pages most often mentioned for the targets.
    ///
    /// `POST /v3/ai_optimization/llm_mentions/top_mentioned_pages/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/top_mentioned_pages/live/>.
    pub async fn top_mentioned_pages(
        &self,
        data: Vec<AiOptimizationLlmMentionsTopMentionedPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/top_mentioned_pages/live",
                &data,
            )
            .await
    }

    /// Lighter, lower-cost variant of [`top_mentioned_pages`](Self::top_mentioned_pages).
    ///
    /// `POST /v3/ai_optimization/llm_mentions/top_mentioned_pages_lite/live`
    pub async fn top_mentioned_pages_lite(
        &self,
        data: Vec<AiOptimizationLlmMentionsTopMentionedPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/top_mentioned_pages_lite/live",
                &data,
            )
            .await
    }

    /// Returns the domains most often mentioned for the targets.
    ///
    /// `POST /v3/ai_optimization/llm_mentions/top_mentioned_domains/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/top_mentioned_domains/live/>.
    pub async fn top_mentioned_domains(
        &self,
        data: Vec<AiOptimizationLlmMentionsTopMentionedPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/top_mentioned_domains/live",
                &data,
            )
            .await
    }

    /// Lighter, lower-cost variant of [`top_mentioned_domains`](Self::top_mentioned_domains).
    ///
    /// `POST /v3/ai_optimization/llm_mentions/top_mentioned_domains_lite/live`
    pub async fn top_mentioned_domains_lite(
        &self,
        data: Vec<AiOptimizationLlmMentionsTopMentionedPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/top_mentioned_domains_lite/live",
                &data,
            )
            .await
    }

    /// Returns the brands most often mentioned for the targets.
    ///
    /// `POST /v3/ai_optimization/llm_mentions/top_mentioned_brands/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/top_mentioned_brands/live/>.
    pub async fn top_mentioned_brands(
        &self,
        data: Vec<AiOptimizationLlmMentionsTopMentionedPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/top_mentioned_brands/live",
                &data,
            )
            .await
    }

    /// Lighter, lower-cost variant of [`top_mentioned_brands`](Self::top_mentioned_brands).
    ///
    /// `POST /v3/ai_optimization/llm_mentions/top_mentioned_brands_lite/live`
    pub async fn top_mentioned_brands_lite(
        &self,
        data: Vec<AiOptimizationLlmMentionsTopMentionedPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/top_mentioned_brands_lite/live",
                &data,
            )
            .await
    }

    /// Returns the brand categories most often mentioned for the targets.
    ///
    /// `POST /v3/ai_optimization/llm_mentions/top_mentioned_brand_categories/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/top_mentioned_brand_categories/live/>.
    pub async fn top_mentioned_brand_categories(
        &self,
        data: Vec<AiOptimizationLlmMentionsTopMentionedPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/top_mentioned_brand_categories/live",
                &data,
            )
            .await
    }

    /// Lighter variant of [`top_mentioned_brand_categories`](Self::top_mentioned_brand_categories).
    ///
    /// `POST /v3/ai_optimization/llm_mentions/top_mentioned_brand_categories_lite/live`
    pub async fn top_mentioned_brand_categories_lite(
        &self,
        data: Vec<AiOptimizationLlmMentionsTopMentionedPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsMetrics> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/top_mentioned_brand_categories_lite/live",
                &data,
            )
            .await
    }

    /// Returns month-by-month historical mention metrics for the targets.
    ///
    /// `POST /v3/ai_optimization/llm_mentions/historical/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/historical/live/>.
    pub async fn historical(
        &self,
        data: Vec<AiOptimizationLlmMentionsTimeseriesPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsTimeseries> {
        self.client
            .http_post("/v3/ai_optimization/llm_mentions/historical/live", &data)
            .await
    }

    /// Returns the period-over-period delta of mention metrics.
    ///
    /// `POST /v3/ai_optimization/llm_mentions/timeseries_delta/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/timeseries_delta/live/>.
    pub async fn timeseries_delta(
        &self,
        data: Vec<AiOptimizationLlmMentionsTimeseriesPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsTimeseries> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/timeseries_delta/live",
                &data,
            )
            .await
    }

    /// Returns newly gained and lost mentions per period.
    ///
    /// `POST /v3/ai_optimization/llm_mentions/timeseries_new_lost/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/timeseries_new_lost/live/>.
    pub async fn timeseries_new_lost(
        &self,
        data: Vec<AiOptimizationLlmMentionsTimeseriesPost>,
    ) -> DataForSeoApiResponse<AiOptimizationLlmMentionsTimeseries> {
        self.client
            .http_post(
                "/v3/ai_optimization/llm_mentions/timeseries_new_lost/live",
                &data,
            )
            .await
    }
}

/// A target entity (domain or keyword) with its search parameters.
///
/// Build with [`domain`](Self::domain) or [`keyword`](Self::keyword).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMentionsTarget {
    /// Target domain (without `https://` or `www`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Target keyword (max 250 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// Whether to `"include"` or `"exclude"` this entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<String>,
    /// Scopes to match in (`any`, `sources`, `search_results`, `question`, `answer`, ...).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_scope: Option<Vec<String>>,
    /// Keyword match type (`word_match` or `partial_match`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
    /// Whether to include subdomains of a domain target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
}

impl AiOptimizationLlmMentionsTarget {
    /// Creates a domain target.
    pub fn domain(domain: &str) -> AiOptimizationLlmMentionsTarget {
        AiOptimizationLlmMentionsTarget {
            domain: Some(domain.to_string()),
            ..Default::default()
        }
    }

    /// Creates a keyword target.
    pub fn keyword(keyword: &str) -> AiOptimizationLlmMentionsTarget {
        AiOptimizationLlmMentionsTarget {
            keyword: Some(keyword.to_string()),
            ..Default::default()
        }
    }
}

/// Request body for `search/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMentionsSearchPost {
    /// Target entities to search for (up to 10).
    pub target: Vec<AiOptimizationLlmMentionsTarget>,
    /// Full name of the search location (default: United States).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Search location code (default: `2840`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full name of the search language (default: English).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Search language code (default: `"en"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Platform to query (`chat_gpt` or `google`; default: `google`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Result filters (max 8), as DataForSEO filter expressions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<serde_json::Value>>,
    /// Sorting rules (max 3), each `"field,direction"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Offset of the first returned result (max 9,000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Token to page beyond 20,000 results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_after_token: Option<String>,
    /// Maximum number of results (default: 100, max: 1,000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl AiOptimizationLlmMentionsSearchPost {
    /// Creates a request for the given targets.
    pub fn new(
        target: Vec<AiOptimizationLlmMentionsTarget>,
    ) -> AiOptimizationLlmMentionsSearchPost {
        AiOptimizationLlmMentionsSearchPost {
            target,
            ..Default::default()
        }
    }
}

/// Request body for `target_metrics/live` (and its `_lite` variant).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMentionsMetricsPost {
    /// Target entities to aggregate metrics for (up to 10).
    pub target: Vec<AiOptimizationLlmMentionsTarget>,
    /// Full name of the search location (default: United States).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Search location code (default: `2840`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full name of the search language (default: English).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Search language code (default: `"en"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Platform to query (`chat_gpt` or `google`; default: `google`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Filters applied before aggregation (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_dataset_filters: Option<Vec<serde_json::Value>>,
    /// Maximum elements kept in each internal breakdown array (1–10, default: 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<u32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl AiOptimizationLlmMentionsMetricsPost {
    /// Creates a request for the given targets.
    pub fn new(
        target: Vec<AiOptimizationLlmMentionsTarget>,
    ) -> AiOptimizationLlmMentionsMetricsPost {
        AiOptimizationLlmMentionsMetricsPost {
            target,
            ..Default::default()
        }
    }
}

/// A named group of targets used by `multi_target_metrics/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMentionsNamedTarget {
    /// Grouping identifier for this target set (max 250 characters).
    pub key: String,
    /// Target entities in this group (up to 10).
    pub target: Vec<AiOptimizationLlmMentionsTarget>,
}

impl AiOptimizationLlmMentionsNamedTarget {
    /// Creates a named target group.
    pub fn new(
        key: &str,
        target: Vec<AiOptimizationLlmMentionsTarget>,
    ) -> AiOptimizationLlmMentionsNamedTarget {
        AiOptimizationLlmMentionsNamedTarget {
            key: key.to_string(),
            target,
        }
    }
}

/// Request body for `multi_target_metrics/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMentionsMultiTargetMetricsPost {
    /// Named target groups to compare (2–10).
    pub targets: Vec<AiOptimizationLlmMentionsNamedTarget>,
    /// Full name of the search location (default: United States).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Search location code (default: `2840`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full name of the search language (default: English).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Search language code (default: `"en"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Platform to query (`chat_gpt` or `google`; default: `google`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Result filters (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<serde_json::Value>>,
    /// Filters applied before aggregation (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_dataset_filters: Option<Vec<serde_json::Value>>,
    /// Sorting rules (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of results (default: 100, max: 1,000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Offset of the first returned result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Maximum elements kept in each internal breakdown array (1–10, default: 5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<u32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl AiOptimizationLlmMentionsMultiTargetMetricsPost {
    /// Creates a request for the given named target groups.
    pub fn new(
        targets: Vec<AiOptimizationLlmMentionsNamedTarget>,
    ) -> AiOptimizationLlmMentionsMultiTargetMetricsPost {
        AiOptimizationLlmMentionsMultiTargetMetricsPost {
            targets,
            ..Default::default()
        }
    }
}

/// Request body shared by the `top_mentioned_*` endpoints (and their `_lite` variants).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMentionsTopMentionedPost {
    /// Target entities to rank mentions for (up to 10).
    pub target: Vec<AiOptimizationLlmMentionsTarget>,
    /// Full name of the search location (default: United States).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Search location code (default: `2840`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full name of the search language (default: English).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Search language code (default: `"en"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Platform to query (`chat_gpt` or `google`; default: `google`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Which links to rank by (`sources` or `search_results`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links_scope: Option<String>,
    /// Result filters (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<serde_json::Value>>,
    /// Filters applied before aggregation (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_dataset_filters: Option<Vec<serde_json::Value>>,
    /// Sorting rules (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Offset of the first returned result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Maximum elements kept in each internal breakdown array (1–10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<u32>,
    /// Restrict results to these domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_domains: Option<Vec<String>>,
    /// Exclude these domains from results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<Vec<String>>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl AiOptimizationLlmMentionsTopMentionedPost {
    /// Creates a request for the given targets.
    pub fn new(
        target: Vec<AiOptimizationLlmMentionsTarget>,
    ) -> AiOptimizationLlmMentionsTopMentionedPost {
        AiOptimizationLlmMentionsTopMentionedPost {
            target,
            ..Default::default()
        }
    }
}

/// Request body shared by `historical/live`, `timeseries_delta/live` and
/// `timeseries_new_lost/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLlmMentionsTimeseriesPost {
    /// Target entities to build the series for (up to 10).
    pub target: Vec<AiOptimizationLlmMentionsTarget>,
    /// Start date (`yyyy-mm-dd`, minimum `2025-08-01`).
    pub date_from: String,
    /// End date (`yyyy-mm-dd`).
    pub date_to: String,
    /// Grouping range (`day`, `week`, `month`, `year`).
    pub group_range: String,
    /// Full name of the search location (default: United States).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Search location code (default: `2840`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full name of the search language (default: English).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Search language code (default: `"en"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Platform to query (`chat_gpt` or `google`; default: `google`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl AiOptimizationLlmMentionsTimeseriesPost {
    /// Creates a request for the given targets and date range.
    pub fn new(
        target: Vec<AiOptimizationLlmMentionsTarget>,
        date_from: &str,
        date_to: &str,
        group_range: &str,
    ) -> AiOptimizationLlmMentionsTimeseriesPost {
        AiOptimizationLlmMentionsTimeseriesPost {
            target,
            date_from: date_from.to_string(),
            date_to: date_to.to_string(),
            group_range: group_range.to_string(),
            ..Default::default()
        }
    }
}
