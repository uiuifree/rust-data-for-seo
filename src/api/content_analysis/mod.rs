use crate::entity::{
    ContentAnalysisApiAvailableFilters, ContentAnalysisApiCategory,
    ContentAnalysisApiCategoryTrends, ContentAnalysisApiIdList, ContentAnalysisApiLanguage,
    ContentAnalysisApiLocation, ContentAnalysisApiPhraseTrends,
    ContentAnalysisApiRatingDistribution, ContentAnalysisApiSearch,
    ContentAnalysisApiSentimentAnalysis, ContentAnalysisApiSummary,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Content Analysis API — citation search and sentiment analytics.
/// See <https://docs.dataforseo.com/v3/content_analysis/overview/>.
pub struct ContentAnalysisApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Returns the Content Analysis API endpoints.
    pub fn content_analysis(&self) -> ContentAnalysisApi<'_> {
        ContentAnalysisApi { client: self }
    }
}

impl ContentAnalysisApi<'_> {
    /// Finds citations for the target keyword.
    /// <https://docs.dataforseo.com/v3/content_analysis/search/live/>
    pub async fn search(
        &self,
        data: Vec<ContentAnalysisApiSearchPost>,
    ) -> DataForSeoApiResponse<ContentAnalysisApiSearch> {
        self.client
            .http_post("/v3/content_analysis/search/live", &data)
            .await
    }

    /// Overview of the citation data available for the target keyword.
    /// <https://docs.dataforseo.com/v3/content_analysis/summary/live/>
    pub async fn summary(
        &self,
        data: Vec<ContentAnalysisApiSummaryPost>,
    ) -> DataForSeoApiResponse<ContentAnalysisApiSummary> {
        self.client
            .http_post("/v3/content_analysis/summary/live", &data)
            .await
    }

    /// Citation stats by sentiment polarity and connotation.
    /// <https://docs.dataforseo.com/v3/content_analysis/sentiment_analysis/live/>
    pub async fn sentiment_analysis(
        &self,
        data: Vec<ContentAnalysisApiSentimentAnalysisPost>,
    ) -> DataForSeoApiResponse<ContentAnalysisApiSentimentAnalysis> {
        self.client
            .http_post("/v3/content_analysis/sentiment_analysis/live", &data)
            .await
    }

    /// Citation stats distributed by content rating.
    /// <https://docs.dataforseo.com/v3/content_analysis/rating_distribution/live/>
    pub async fn rating_distribution(
        &self,
        data: Vec<ContentAnalysisApiRatingDistributionPost>,
    ) -> DataForSeoApiResponse<ContentAnalysisApiRatingDistribution> {
        self.client
            .http_post("/v3/content_analysis/rating_distribution/live", &data)
            .await
    }

    /// Citation stats by date for the target keyword.
    /// <https://docs.dataforseo.com/v3/content_analysis/phrase_trends/live/>
    pub async fn phrase_trends(
        &self,
        data: Vec<ContentAnalysisApiPhraseTrendsPost>,
    ) -> DataForSeoApiResponse<ContentAnalysisApiPhraseTrends> {
        self.client
            .http_post("/v3/content_analysis/phrase_trends/live", &data)
            .await
    }

    /// Citation stats by date for the target category.
    /// <https://docs.dataforseo.com/v3/content_analysis/category_trends/live/>
    pub async fn category_trends(
        &self,
        data: Vec<ContentAnalysisApiCategoryTrendsPost>,
    ) -> DataForSeoApiResponse<ContentAnalysisApiCategoryTrends> {
        self.client
            .http_post("/v3/content_analysis/category_trends/live", &data)
            .await
    }

    /// Lists tasks set within the specified datetime range.
    /// <https://docs.dataforseo.com/v3/content_analysis/id_list/>
    pub async fn id_list(
        &self,
        data: Vec<ContentAnalysisApiIdListPost>,
    ) -> DataForSeoApiResponse<ContentAnalysisApiIdList> {
        self.client
            .http_post("/v3/content_analysis/id_list", &data)
            .await
    }

    /// Categories available in the content-analysis taxonomy.
    /// <https://docs.dataforseo.com/v3/content_analysis/categories/>
    pub async fn categories(&self) -> DataForSeoApiResponse<ContentAnalysisApiCategory> {
        self.client
            .http_get("/v3/content_analysis/categories")
            .await
    }

    /// Locations supported by the Content Analysis API.
    /// <https://docs.dataforseo.com/v3/content_analysis/locations/>
    pub async fn locations(&self) -> DataForSeoApiResponse<ContentAnalysisApiLocation> {
        self.client.http_get("/v3/content_analysis/locations").await
    }

    /// Languages supported by the Content Analysis API.
    /// <https://docs.dataforseo.com/v3/content_analysis/languages/>
    pub async fn languages(&self) -> DataForSeoApiResponse<ContentAnalysisApiLanguage> {
        self.client.http_get("/v3/content_analysis/languages").await
    }

    /// Filters that can be used in `filters` / `initial_dataset_filters`.
    /// <https://docs.dataforseo.com/v3/content_analysis/available_filters/>
    pub async fn available_filters(
        &self,
    ) -> DataForSeoApiResponse<ContentAnalysisApiAvailableFilters> {
        self.client
            .http_get("/v3/content_analysis/available_filters")
            .await
    }
}

/// Request for `content_analysis/search/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiSearchPost {
    /// Target keyword (UTF-8, lowercased); wrap in escaped double quotes for an
    /// exact phrase.
    pub keyword: String,
    /// Restrict matches to keywords appearing in given fields (title,
    /// main_title, previous_title, snippet).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_fields: Option<HashMap<String, String>>,
    /// Filter by page types (ecommerce, news, blogs, message-boards,
    /// organization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_type: Option<Vec<String>>,
    /// Results grouping: `as_is` (default) or `one_per_domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_mode: Option<String>,
    /// Maximum citations to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Up to 8 filtering conditions joined by `and`/`or`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Up to 3 sorting rules, e.g. `content_info.sentiment_connotations.anger,desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Number of leading citations to skip; use for up to 10,000 results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Token from a previous response to page beyond 10,000 results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_token: Option<String>,
    /// Scale for rank values: `one_hundred` or `one_thousand` (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentAnalysisApiSearchPost {
    /// Creates a request for the given target keyword.
    pub fn new(keyword: &str) -> ContentAnalysisApiSearchPost {
        ContentAnalysisApiSearchPost {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_analysis/summary/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiSummaryPost {
    /// Target keyword (UTF-8, lowercased); wrap in escaped double quotes for an
    /// exact phrase.
    pub keyword: String,
    /// Restrict matches to keywords appearing in given fields (title,
    /// main_title, previous_title, snippet).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_fields: Option<HashMap<String, String>>,
    /// Filter by page types (ecommerce, news, blogs, message-boards,
    /// organization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_type: Option<Vec<String>>,
    /// Maximum elements within internal arrays like top_domains and countries
    /// (default 1, max 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<u32>,
    /// Minimum positive-sentiment probability for a citation to count
    /// (0–1, default 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_connotation_threshold: Option<f64>,
    /// Minimum per-sentiment probability for a citation to count
    /// (0–1, default 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiments_connotation_threshold: Option<f64>,
    /// Up to 8 filtering conditions applied to Search fields before aggregation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_dataset_filters: Option<Vec<Value>>,
    /// Scale for rank values: `one_hundred` or `one_thousand` (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentAnalysisApiSummaryPost {
    /// Creates a request for the given target keyword.
    pub fn new(keyword: &str) -> ContentAnalysisApiSummaryPost {
        ContentAnalysisApiSummaryPost {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_analysis/sentiment_analysis/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiSentimentAnalysisPost {
    /// Target keyword (UTF-8, lowercased); wrap in escaped double quotes for an
    /// exact phrase.
    pub keyword: String,
    /// Restrict matches to keywords appearing in given fields (title,
    /// main_title, previous_title, snippet).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_fields: Option<HashMap<String, String>>,
    /// Filter by page types (ecommerce, news, blogs, message-boards,
    /// organization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_type: Option<Vec<String>>,
    /// Maximum elements within internal arrays like top_domains and countries
    /// (default 1, max 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<u32>,
    /// Minimum positive-sentiment probability for a citation to count
    /// (0–1, default 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_connotation_threshold: Option<f64>,
    /// Minimum per-sentiment probability for a citation to count
    /// (0–1, default 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiments_connotation_threshold: Option<f64>,
    /// Up to 8 filtering conditions applied to Search fields before aggregation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_dataset_filters: Option<Vec<Value>>,
    /// Scale for rank values: `one_hundred` or `one_thousand` (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentAnalysisApiSentimentAnalysisPost {
    /// Creates a request for the given target keyword.
    pub fn new(keyword: &str) -> ContentAnalysisApiSentimentAnalysisPost {
        ContentAnalysisApiSentimentAnalysisPost {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_analysis/rating_distribution/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiRatingDistributionPost {
    /// Target keyword (UTF-8, lowercased); wrap in escaped double quotes for an
    /// exact phrase.
    pub keyword: String,
    /// Restrict matches to keywords appearing in given fields (title,
    /// main_title, previous_title, snippet).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_fields: Option<HashMap<String, String>>,
    /// Filter by page types (ecommerce, news, blogs, message-boards,
    /// organization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_type: Option<Vec<String>>,
    /// Maximum elements within internal arrays like top_domains and countries
    /// (default 1, max 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<u32>,
    /// Results grouping: `as_is` (default) or `one_per_domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_mode: Option<String>,
    /// Minimum positive-sentiment probability for a citation to count
    /// (0–1, default 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_connotation_threshold: Option<f64>,
    /// Minimum per-sentiment probability for a citation to count
    /// (0–1, default 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiments_connotation_threshold: Option<f64>,
    /// Up to 8 filtering conditions applied to Search fields before aggregation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_dataset_filters: Option<Vec<Value>>,
    /// Scale for rank values: `one_hundred` or `one_thousand` (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentAnalysisApiRatingDistributionPost {
    /// Creates a request for the given target keyword.
    pub fn new(keyword: &str) -> ContentAnalysisApiRatingDistributionPost {
        ContentAnalysisApiRatingDistributionPost {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_analysis/phrase_trends/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiPhraseTrendsPost {
    /// Target keyword (UTF-8, lowercased); wrap in escaped double quotes for an
    /// exact phrase.
    pub keyword: String,
    /// Start of the date range, `yyyy-mm-dd`.
    pub date_from: String,
    /// End of the date range, `yyyy-mm-dd` (defaults to today).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Grouping interval: `day`, `week`, or `month` (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_group: Option<String>,
    /// Restrict matches to keywords appearing in given fields (title,
    /// main_title, previous_title, snippet).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_fields: Option<HashMap<String, String>>,
    /// Filter by page types (ecommerce, news, blogs, message-boards,
    /// organization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_type: Option<Vec<String>>,
    /// Results grouping: `as_is` (default) or `one_per_domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_mode: Option<String>,
    /// Maximum elements within internal arrays like top_domains and countries
    /// (default 1, max 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<u32>,
    /// Up to 8 filtering conditions applied to Search fields before aggregation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_dataset_filters: Option<Vec<Value>>,
    /// Scale for rank values: `one_hundred` or `one_thousand` (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentAnalysisApiPhraseTrendsPost {
    /// Creates a request for the given keyword and start date (`yyyy-mm-dd`).
    pub fn new(keyword: &str, date_from: &str) -> ContentAnalysisApiPhraseTrendsPost {
        ContentAnalysisApiPhraseTrendsPost {
            keyword: keyword.to_string(),
            date_from: date_from.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_analysis/category_trends/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiCategoryTrendsPost {
    /// Target category code; obtain it from the Categories endpoint.
    pub category_code: i64,
    /// Start of the date range, `yyyy-mm-dd` (minimum 2022-10-31).
    pub date_from: String,
    /// End of the date range, `yyyy-mm-dd` (defaults to today).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Grouping interval: `day`, `week`, or `month` (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_group: Option<String>,
    /// Filter by page types (ecommerce, news, blogs, message-boards,
    /// organization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_type: Option<Vec<String>>,
    /// Results grouping: `as_is` (default) or `one_per_domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_mode: Option<String>,
    /// Maximum elements within internal arrays like top_domains and countries
    /// (default 1, max 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<u32>,
    /// Up to 8 filtering conditions applied to Search fields before aggregation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_dataset_filters: Option<Vec<Value>>,
    /// Scale for rank values: `one_hundred` or `one_thousand` (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentAnalysisApiCategoryTrendsPost {
    /// Creates a request for the given category code and start date (`yyyy-mm-dd`).
    pub fn new(category_code: i64, date_from: &str) -> ContentAnalysisApiCategoryTrendsPost {
        ContentAnalysisApiCategoryTrendsPost {
            category_code,
            date_from: date_from.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_analysis/id_list`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiIdListPost {
    /// Start of the range, UTC `yyyy-mm-dd hh-mm-ss +00:00`.
    pub datetime_from: String,
    /// End of the range, UTC `yyyy-mm-dd hh-mm-ss +00:00`.
    pub datetime_to: String,
    /// Maximum task IDs to return (default 1000, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Number of leading task IDs to skip (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Sort by task execution time: `asc` (default) or `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Include each task's request metadata (default false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
}

impl ContentAnalysisApiIdListPost {
    /// Creates a request for the given datetime range
    /// (`yyyy-mm-dd hh-mm-ss +00:00`).
    pub fn new(datetime_from: &str, datetime_to: &str) -> ContentAnalysisApiIdListPost {
        ContentAnalysisApiIdListPost {
            datetime_from: datetime_from.to_string(),
            datetime_to: datetime_to.to_string(),
            ..Default::default()
        }
    }
}
