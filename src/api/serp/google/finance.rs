use crate::entity::{
    SerpApiGoogleFinanceExploreTaskAdvanced, SerpApiGoogleFinanceExploreTaskHtml,
    SerpApiGoogleFinanceMarketsTaskAdvanced, SerpApiGoogleFinanceMarketsTaskHtml,
    SerpApiGoogleFinanceQuoteTaskAdvanced, SerpApiGoogleFinanceTickerSearchTaskAdvanced,
};
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Finance SERP endpoints (Explore, Ticker Search, Markets, Quote).
/// <https://docs.dataforseo.com/v3/serp/google/finance/overview/>
impl SerpApiGoogle<'_> {
    // --- Explore -----------------------------------------------------------

    /// <https://docs.dataforseo.com/v3/serp/google/finance_explore/task_post/>
    pub async fn finance_explore_task_post(
        &self,
        data: Vec<SerpApiGoogleFinanceExploreTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/finance_explore/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_explore/tasks_ready/>
    pub async fn finance_explore_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/finance_explore")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_explore/task_get/advanced/>
    pub async fn finance_explore_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleFinanceExploreTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/finance_explore/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_explore/task_get/html/>
    pub async fn finance_explore_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleFinanceExploreTaskHtml> {
        self.client
            .http_get(format!("/v3/serp/google/finance_explore/task_get/html/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_explore/live/advanced/>
    pub async fn finance_explore_live_advanced(
        &self,
        data: Vec<SerpApiGoogleFinanceExploreTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleFinanceExploreTaskAdvanced> {
        self.client
            .http_post("/v3/serp/google/finance_explore/live/advanced", &data)
            .await
    }

    // --- Ticker Search -----------------------------------------------------

    /// <https://docs.dataforseo.com/v3/serp/google/finance_ticker_search/task_post/>
    pub async fn finance_ticker_search_task_post(
        &self,
        data: Vec<SerpApiGoogleFinanceTickerSearchTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/finance_ticker_search/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_ticker_search/tasks_ready/>
    pub async fn finance_ticker_search_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/finance_ticker_search")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_ticker_search/task_get/advanced/>
    pub async fn finance_ticker_search_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleFinanceTickerSearchTaskAdvanced> {
        self.client
            .http_get(
                format!("/v3/serp/google/finance_ticker_search/task_get/advanced/{id}").as_str(),
            )
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_ticker_search/live/advanced/>
    pub async fn finance_ticker_search_live_advanced(
        &self,
        data: Vec<SerpApiGoogleFinanceTickerSearchTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleFinanceTickerSearchTaskAdvanced> {
        self.client
            .http_post("/v3/serp/google/finance_ticker_search/live/advanced", &data)
            .await
    }

    // --- Markets -----------------------------------------------------------

    /// <https://docs.dataforseo.com/v3/serp/google/finance_markets/task_post/>
    pub async fn finance_markets_task_post(
        &self,
        data: Vec<SerpApiGoogleFinanceMarketsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/finance_markets/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_markets/tasks_ready/>
    pub async fn finance_markets_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/finance_markets")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_markets/task_get/advanced/>
    pub async fn finance_markets_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleFinanceMarketsTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/finance_markets/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_markets/task_get/html/>
    pub async fn finance_markets_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleFinanceMarketsTaskHtml> {
        self.client
            .http_get(format!("/v3/serp/google/finance_markets/task_get/html/{id}").as_str())
            .await
    }

    // --- Quote -------------------------------------------------------------

    /// <https://docs.dataforseo.com/v3/serp/google/finance_quote/task_post/>
    pub async fn finance_quote_task_post(
        &self,
        data: Vec<SerpApiGoogleFinanceQuoteTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/finance_quote/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_quote/tasks_ready/>
    pub async fn finance_quote_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/finance_quote")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/finance_quote/task_get/advanced/>
    pub async fn finance_quote_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleFinanceQuoteTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/finance_quote/task_get/advanced/{id}").as_str())
            .await
    }
}

/// Request body for Google Finance Explore task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/finance_explore/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleFinanceExploreTaskPostRequest {
    /// Full location name (e.g. "London,England,United Kingdom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: i32,
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    pub language_code: String,
    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// News type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news_type: Option<String>,
    /// User-defined identifier echoed back on the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL of the postback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Postback data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL of the pingback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl SerpApiGoogleFinanceExploreTaskPostRequest {
    /// Creates a request with the required language and location codes.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleFinanceExploreTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleFinanceExploreTaskPostRequest::default()
        }
    }
}

/// Request body for Google Finance Ticker Search task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/finance_ticker_search/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleFinanceTickerSearchTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
    /// Full location name (e.g. "London,England,United Kingdom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: i32,
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    pub language_code: String,
    /// Category of the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined identifier echoed back on the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL of the postback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Postback data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL of the pingback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl SerpApiGoogleFinanceTickerSearchTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` (company / instrument name) before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleFinanceTickerSearchTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleFinanceTickerSearchTaskPostRequest::default()
        }
    }
}

/// Request body for Google Finance Markets task_post.
/// See <https://docs.dataforseo.com/v3/serp/google/finance_markets/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleFinanceMarketsTaskPostRequest {
    /// Full location name (e.g. "London,England,United Kingdom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: i32,
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    pub language_code: String,
    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Market type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined identifier echoed back on the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL of the postback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Postback data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL of the pingback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl SerpApiGoogleFinanceMarketsTaskPostRequest {
    /// Creates a request with the required language and location codes.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleFinanceMarketsTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleFinanceMarketsTaskPostRequest::default()
        }
    }
}

/// Request body for Google Finance Quote task_post.
/// See <https://docs.dataforseo.com/v3/serp/google/finance_quote/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleFinanceQuoteTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
    /// Full location name (e.g. "London,England,United Kingdom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: i32,
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    pub language_code: String,
    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined identifier echoed back on the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL of the postback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Postback data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL of the pingback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl SerpApiGoogleFinanceQuoteTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` (ticker / stock symbol) before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleFinanceQuoteTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleFinanceQuoteTaskPostRequest::default()
        }
    }
}
