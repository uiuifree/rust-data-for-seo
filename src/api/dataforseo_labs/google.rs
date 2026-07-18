//! Google endpoints of the DataForSEO Labs API.
//! See <https://docs.dataforseo.com/v3/dataforseo_labs/google/>.

use crate::entity::{
    DataForSeoLabsAppCompetitorItem, DataForSeoLabsAppIntersectionItem,
    DataForSeoLabsAppKeywordItem, DataForSeoLabsBulkAppMetricsItem,
    DataForSeoLabsCategoriesForKeywordsItem, DataForSeoLabsCategoryMetricsItem,
    DataForSeoLabsCompetitorItem, DataForSeoLabsDomainIntersectionItem,
    DataForSeoLabsDomainMetricsByCategoriesItem, DataForSeoLabsDomainMetricsItem,
    DataForSeoLabsHistoricalKeywordDataItem, DataForSeoLabsHistoricalRankOverviewItem,
    DataForSeoLabsHistoricalSerpItem, DataForSeoLabsHistoricalTrafficEstimationItem,
    DataForSeoLabsKeywordData, DataForSeoLabsKeywordDifficultyItem,
    DataForSeoLabsPageIntersectionItem, DataForSeoLabsRankedKeywordItem,
    DataForSeoLabsRelatedKeywordItem, DataForSeoLabsRelevantPageItem, DataForSeoLabsResult,
    DataForSeoLabsSearchIntentItem, DataForSeoLabsSerpCompetitorItem, DataForSeoLabsSubdomainItem,
    DataForSeoLabsTrafficEstimationItem,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Google endpoints of the DataForSEO Labs API.
pub struct DataForSeoLabsApiGoogle<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl DataForSeoLabsApiGoogle<'_> {
    /// Keywords a domain/subdomain/page ranks for in organic and paid search.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/ranked_keywords/live/>.
    pub async fn ranked_keywords(
        &self,
        data: Vec<DataForSeoLabsRankedKeywordsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsRankedKeywordItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/ranked_keywords/live", &data)
            .await
    }

    /// Keyword ideas relevant to the provided seed keywords.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_ideas/live/>.
    pub async fn keyword_ideas(
        &self,
        data: Vec<DataForSeoLabsKeywordIdeasRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsKeywordData>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/keyword_ideas/live", &data)
            .await
    }

    /// Keyword suggestions that include the seed keyword as a substring.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_suggestions/live/>.
    pub async fn keyword_suggestions(
        &self,
        data: Vec<DataForSeoLabsKeywordSuggestionsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsKeywordData>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/keyword_suggestions/live", &data)
            .await
    }

    /// Keywords related to a seed keyword, discovered via "searches related to".
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/related_keywords/live/>.
    pub async fn related_keywords(
        &self,
        data: Vec<DataForSeoLabsRelatedKeywordsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsRelatedKeywordItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/related_keywords/live", &data)
            .await
    }

    /// Keywords a website is known to be relevant for.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_site/live/>.
    pub async fn keywords_for_site(
        &self,
        data: Vec<DataForSeoLabsKeywordsForSiteRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsKeywordData>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/keywords_for_site/live", &data)
            .await
    }

    /// Keyword difficulty for a batch of keywords.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/bulk_keyword_difficulty/live/>.
    pub async fn bulk_keyword_difficulty(
        &self,
        data: Vec<DataForSeoLabsBulkKeywordDifficultyRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsKeywordDifficultyItem>> {
        self.client
            .http_post(
                "/v3/dataforseo_labs/google/bulk_keyword_difficulty/live",
                &data,
            )
            .await
    }

    /// Search-intent classification for a batch of keywords.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/search_intent/live/>.
    pub async fn search_intent(
        &self,
        data: Vec<DataForSeoLabsSearchIntentRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsSearchIntentItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/search_intent/live", &data)
            .await
    }

    /// Full keyword metrics (overview) for a batch of keywords.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_overview/live/>.
    pub async fn keyword_overview(
        &self,
        data: Vec<DataForSeoLabsKeywordOverviewRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsKeywordData>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/keyword_overview/live", &data)
            .await
    }

    /// Historical monthly Google Ads metrics for a batch of keywords.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_keyword_data/live/>.
    pub async fn historical_keyword_data(
        &self,
        data: Vec<DataForSeoLabsHistoricalKeywordDataRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsHistoricalKeywordDataItem>> {
        self.client
            .http_post(
                "/v3/dataforseo_labs/google/historical_keyword_data/live",
                &data,
            )
            .await
    }

    /// Historical SERP snapshots for a keyword.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_serps/live/>.
    pub async fn historical_serps(
        &self,
        data: Vec<DataForSeoLabsHistoricalSerpsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsHistoricalSerpItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/historical_serps/live", &data)
            .await
    }

    /// Most popular keywords for a location/language.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/top_searches/live/>.
    pub async fn top_searches(
        &self,
        data: Vec<DataForSeoLabsTopSearchesRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsKeywordData>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/top_searches/live", &data)
            .await
    }

    /// Domains competing for a set of keywords in the live SERP.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/serp_competitors/live/>.
    pub async fn serp_competitors(
        &self,
        data: Vec<DataForSeoLabsSerpCompetitorsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsSerpCompetitorItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/serp_competitors/live", &data)
            .await
    }

    /// Domains that compete with a target across their organic keywords.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/competitors_domain/live/>.
    pub async fn competitors_domain(
        &self,
        data: Vec<DataForSeoLabsCompetitorsDomainRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsCompetitorItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/competitors_domain/live", &data)
            .await
    }

    /// Keywords two domains both rank for, with each domain's SERP element.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_intersection/live/>.
    pub async fn domain_intersection(
        &self,
        data: Vec<DataForSeoLabsDomainIntersectionRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsDomainIntersectionItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/domain_intersection/live", &data)
            .await
    }

    /// Subdomains of a target and their ranking metrics.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/subdomains/live/>.
    pub async fn subdomains(
        &self,
        data: Vec<DataForSeoLabsSubdomainsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsSubdomainItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/subdomains/live", &data)
            .await
    }

    /// Pages of a target ranked by the number of keywords they rank for.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/relevant_pages/live/>.
    pub async fn relevant_pages(
        &self,
        data: Vec<DataForSeoLabsRelevantPagesRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsRelevantPageItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/relevant_pages/live", &data)
            .await
    }

    /// Aggregated ranking overview of a domain.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_rank_overview/live/>.
    pub async fn domain_rank_overview(
        &self,
        data: Vec<DataForSeoLabsDomainRankOverviewRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsDomainMetricsItem>> {
        self.client
            .http_post(
                "/v3/dataforseo_labs/google/domain_rank_overview/live",
                &data,
            )
            .await
    }

    /// Keywords for which a set of pages appear together in the SERP.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/page_intersection/live/>.
    pub async fn page_intersection(
        &self,
        data: Vec<DataForSeoLabsPageIntersectionRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsPageIntersectionItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/page_intersection/live", &data)
            .await
    }

    /// Estimated organic/paid traffic for a batch of targets.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/bulk_traffic_estimation/live/>.
    pub async fn bulk_traffic_estimation(
        &self,
        data: Vec<DataForSeoLabsBulkTrafficEstimationRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsTrafficEstimationItem>> {
        self.client
            .http_post(
                "/v3/dataforseo_labs/google/bulk_traffic_estimation/live",
                &data,
            )
            .await
    }

    /// Historical estimated traffic for a batch of targets.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_bulk_traffic_estimation/live/>.
    pub async fn historical_bulk_traffic_estimation(
        &self,
        data: Vec<DataForSeoLabsHistoricalBulkTrafficEstimationRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsHistoricalTrafficEstimationItem>>
    {
        self.client
            .http_post(
                "/v3/dataforseo_labs/google/historical_bulk_traffic_estimation/live",
                &data,
            )
            .await
    }

    /// Monthly historical ranking overview of a domain.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_rank_overview/live/>.
    pub async fn historical_rank_overview(
        &self,
        data: Vec<DataForSeoLabsHistoricalRankOverviewRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsHistoricalRankOverviewItem>> {
        self.client
            .http_post(
                "/v3/dataforseo_labs/google/historical_rank_overview/live",
                &data,
            )
            .await
    }

    /// Product categories a domain ranks in, with per-category metrics.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/categories_for_domain/live/>.
    pub async fn categories_for_domain(
        &self,
        data: Vec<DataForSeoLabsCategoriesForDomainRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsCategoryMetricsItem>> {
        self.client
            .http_post(
                "/v3/dataforseo_labs/google/categories_for_domain/live",
                &data,
            )
            .await
    }

    /// Product categories associated with a batch of keywords.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/categories_for_keywords/live/>.
    pub async fn categories_for_keywords(
        &self,
        data: Vec<DataForSeoLabsCategoriesForKeywordsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsCategoriesForKeywordsItem>> {
        self.client
            .http_post(
                "/v3/dataforseo_labs/google/categories_for_keywords/live",
                &data,
            )
            .await
    }

    /// Keywords that belong to a set of product categories.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_categories/live/>.
    pub async fn keywords_for_categories(
        &self,
        data: Vec<DataForSeoLabsKeywordsForCategoriesRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsKeywordData>> {
        self.client
            .http_post(
                "/v3/dataforseo_labs/google/keywords_for_categories/live",
                &data,
            )
            .await
    }

    /// Domains ranking within a set of product categories, with their metrics.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_metrics_by_categories/live/>.
    pub async fn domain_metrics_by_categories(
        &self,
        data: Vec<DataForSeoLabsDomainMetricsByCategoriesRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsDomainMetricsByCategoriesItem>>
    {
        self.client
            .http_post(
                "/v3/dataforseo_labs/google/domain_metrics_by_categories/live",
                &data,
            )
            .await
    }

    /// Keywords a Google Play app ranks for.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_app/live/>.
    pub async fn keywords_for_app(
        &self,
        data: Vec<DataForSeoLabsKeywordsForAppRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAppKeywordItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/keywords_for_app/live", &data)
            .await
    }

    /// Google Play apps competing with a target app.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/app_competitors/live/>.
    pub async fn app_competitors(
        &self,
        data: Vec<DataForSeoLabsAppCompetitorsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAppCompetitorItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/app_competitors/live", &data)
            .await
    }

    /// Keywords a set of Google Play apps rank for together.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/app_intersection/live/>.
    pub async fn app_intersection(
        &self,
        data: Vec<DataForSeoLabsAppIntersectionRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAppIntersectionItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/app_intersection/live", &data)
            .await
    }

    /// Aggregated metrics for a batch of Google Play apps.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/bulk_app_metrics/live/>.
    pub async fn bulk_app_metrics(
        &self,
        data: Vec<DataForSeoLabsBulkAppMetricsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsBulkAppMetricsItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/google/bulk_app_metrics/live", &data)
            .await
    }
}

/// Request body for `ranked_keywords`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsRankedKeywordsRequest {
    /// Domain, subdomain, or webpage the request targets.
    pub target: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Whether to include the absolute rank of each element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_rank_absolute: Option<bool>,
    /// How to treat historical SERP data: `"live"`, `"lost"`, or `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_serp_mode: Option<String>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsRankedKeywordsRequest {
    /// Creates a request for the given target (domain, subdomain or page URL).
    pub fn new(target: &str) -> DataForSeoLabsRankedKeywordsRequest {
        DataForSeoLabsRankedKeywordsRequest {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `keyword_ideas`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_ideas/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordIdeasRequest {
    /// Keywords to analyze.
    pub keywords: Vec<String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether to match only close variants of the seed keywords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closely_variants: Option<bool>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// Whether to include SERP metadata for each keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_serp_info: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Token used to page beyond the standard offset limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsKeywordIdeasRequest {
    /// Creates a request for the given seed keywords.
    pub fn new(keywords: Vec<String>) -> DataForSeoLabsKeywordIdeasRequest {
        DataForSeoLabsKeywordIdeasRequest {
            keywords,
            ..Default::default()
        }
    }
}

/// Request body for `keyword_suggestions`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_suggestions/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordSuggestionsRequest {
    /// Keyword the data relates to.
    pub keyword: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether to include data for the seed keyword itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_seed_keyword: Option<bool>,
    /// Whether to include SERP metadata for each keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_serp_info: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Whether to return only keywords containing the exact seed phrase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_match: Option<bool>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Token used to page beyond the standard offset limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsKeywordSuggestionsRequest {
    /// Creates a request for the given seed keyword.
    pub fn new(keyword: &str) -> DataForSeoLabsKeywordSuggestionsRequest {
        DataForSeoLabsKeywordSuggestionsRequest {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `related_keywords`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/related_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsRelatedKeywordsRequest {
    /// Keyword the data relates to.
    pub keyword: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Keyword expansion depth (0-4); higher values return more keywords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    /// Whether to include data for the seed keyword itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_seed_keyword: Option<bool>,
    /// Whether to include SERP metadata for each keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_serp_info: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// Whether to group results under their core (canonical) keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_with_core_keyword: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsRelatedKeywordsRequest {
    /// Creates a request for the given seed keyword.
    pub fn new(keyword: &str) -> DataForSeoLabsRelatedKeywordsRequest {
        DataForSeoLabsRelatedKeywordsRequest {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `keywords_for_site`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_site/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordsForSiteRequest {
    /// Domain, subdomain, or webpage the request targets.
    pub target: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Type of target: `"domain"`, `"subdomain"`, or `"page"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// Whether to include SERP metadata for each keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_serp_info: Option<bool>,
    /// Whether to include subdomains of the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsKeywordsForSiteRequest {
    /// Creates a request for the given target site.
    pub fn new(target: &str) -> DataForSeoLabsKeywordsForSiteRequest {
        DataForSeoLabsKeywordsForSiteRequest {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `bulk_keyword_difficulty`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/bulk_keyword_difficulty/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsBulkKeywordDifficultyRequest {
    /// Keywords to analyze.
    pub keywords: Vec<String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsBulkKeywordDifficultyRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> DataForSeoLabsBulkKeywordDifficultyRequest {
        DataForSeoLabsBulkKeywordDifficultyRequest {
            keywords,
            ..Default::default()
        }
    }
}

/// Request body for `search_intent`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/search_intent/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSearchIntentRequest {
    /// Keywords to analyze.
    pub keywords: Vec<String>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsSearchIntentRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> DataForSeoLabsSearchIntentRequest {
        DataForSeoLabsSearchIntentRequest {
            keywords,
            ..Default::default()
        }
    }
}

/// Request body for `keyword_overview`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_overview/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordOverviewRequest {
    /// Keywords to analyze.
    pub keywords: Vec<String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether to include SERP metadata for each keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_serp_info: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsKeywordOverviewRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> DataForSeoLabsKeywordOverviewRequest {
        DataForSeoLabsKeywordOverviewRequest {
            keywords,
            ..Default::default()
        }
    }
}

/// Request body for `historical_keyword_data`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_keyword_data/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsHistoricalKeywordDataRequest {
    /// Keywords to analyze.
    pub keywords: Vec<String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsHistoricalKeywordDataRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> DataForSeoLabsHistoricalKeywordDataRequest {
        DataForSeoLabsHistoricalKeywordDataRequest {
            keywords,
            ..Default::default()
        }
    }
}

/// Request body for `historical_serps`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_serps/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsHistoricalSerpsRequest {
    /// Keyword the data relates to.
    pub keyword: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Start date of the range, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End date of the range, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsHistoricalSerpsRequest {
    /// Creates a request for the given keyword.
    pub fn new(keyword: &str) -> DataForSeoLabsHistoricalSerpsRequest {
        DataForSeoLabsHistoricalSerpsRequest {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `top_searches`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/top_searches/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsTopSearchesRequest {
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether to include SERP metadata for each keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_serp_info: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Token used to page beyond the standard offset limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsTopSearchesRequest {
    /// Creates a request for the given location/language (set via the fields).
    pub fn new() -> DataForSeoLabsTopSearchesRequest {
        DataForSeoLabsTopSearchesRequest::default()
    }
}

/// Request body for `serp_competitors`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/serp_competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSerpCompetitorsRequest {
    /// Keywords to analyze.
    pub keywords: Vec<String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether to include subdomains of the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsSerpCompetitorsRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> DataForSeoLabsSerpCompetitorsRequest {
        DataForSeoLabsSerpCompetitorsRequest {
            keywords,
            ..Default::default()
        }
    }
}

/// Request body for `competitors_domain`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/competitors_domain/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsCompetitorsDomainRequest {
    /// Domain, subdomain, or webpage the request targets.
    pub target: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Maximum rank position considered when detecting competitors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rank_group: Option<i64>,
    /// Whether to exclude the most popular domains from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_top_domains: Option<bool>,
    /// Domains to exclude from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<Vec<String>>,
    /// Domains used to refine competitor-detection accuracy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersecting_domains: Option<Vec<String>>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsCompetitorsDomainRequest {
    /// Creates a request for the given target domain.
    pub fn new(target: &str) -> DataForSeoLabsCompetitorsDomainRequest {
        DataForSeoLabsCompetitorsDomainRequest {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `domain_intersection`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_intersection/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsDomainIntersectionRequest {
    /// First domain to compare.
    pub target1: String,
    /// Second domain to compare.
    pub target2: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether to return only keywords both targets rank for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersections: Option<bool>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// Whether to include SERP metadata for each keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_serp_info: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsDomainIntersectionRequest {
    /// Creates a request comparing `target1` and `target2`.
    pub fn new(target1: &str, target2: &str) -> DataForSeoLabsDomainIntersectionRequest {
        DataForSeoLabsDomainIntersectionRequest {
            target1: target1.to_string(),
            target2: target2.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `subdomains`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/subdomains/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSubdomainsRequest {
    /// Domain, subdomain, or webpage the request targets.
    pub target: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// How to treat historical SERP data: `"live"`, `"lost"`, or `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_serp_mode: Option<String>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsSubdomainsRequest {
    /// Creates a request for the given target domain.
    pub fn new(target: &str) -> DataForSeoLabsSubdomainsRequest {
        DataForSeoLabsSubdomainsRequest {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `relevant_pages`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/relevant_pages/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsRelevantPagesRequest {
    /// Domain, subdomain, or webpage the request targets.
    pub target: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsRelevantPagesRequest {
    /// Creates a request for the given target domain.
    pub fn new(target: &str) -> DataForSeoLabsRelevantPagesRequest {
        DataForSeoLabsRelevantPagesRequest {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `domain_rank_overview`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_rank_overview/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsDomainRankOverviewRequest {
    /// Domain, subdomain, or webpage the request targets.
    pub target: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsDomainRankOverviewRequest {
    /// Creates a request for the given target domain.
    pub fn new(target: &str) -> DataForSeoLabsDomainRankOverviewRequest {
        DataForSeoLabsDomainRankOverviewRequest {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `page_intersection`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/page_intersection/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsPageIntersectionRequest {
    /// Target page URLs to intersect, keyed by an index string.
    pub pages: HashMap<String, String>,
    /// Page URLs to exclude from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_pages: Option<Vec<String>>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// How the intersection is computed: `"union"` or `"intersect"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersection_mode: Option<String>,
    /// Whether to include SERP metadata for each keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_serp_info: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsPageIntersectionRequest {
    /// Creates a request over the given pages map (index string -> page URL).
    pub fn new(pages: HashMap<String, String>) -> DataForSeoLabsPageIntersectionRequest {
        DataForSeoLabsPageIntersectionRequest {
            pages,
            ..Default::default()
        }
    }
}

/// Request body for `bulk_traffic_estimation`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/bulk_traffic_estimation/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsBulkTrafficEstimationRequest {
    /// Domains, subdomains, or webpages to analyze (up to 1000).
    pub targets: Vec<String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsBulkTrafficEstimationRequest {
    /// Creates a request for the given targets.
    pub fn new(targets: Vec<String>) -> DataForSeoLabsBulkTrafficEstimationRequest {
        DataForSeoLabsBulkTrafficEstimationRequest {
            targets,
            ..Default::default()
        }
    }
}

/// Request body for `historical_bulk_traffic_estimation`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_bulk_traffic_estimation/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsHistoricalBulkTrafficEstimationRequest {
    /// Domains, subdomains, or webpages to analyze (up to 1000).
    pub targets: Vec<String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Start date of the range, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End date of the range, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsHistoricalBulkTrafficEstimationRequest {
    /// Creates a request for the given targets.
    pub fn new(targets: Vec<String>) -> DataForSeoLabsHistoricalBulkTrafficEstimationRequest {
        DataForSeoLabsHistoricalBulkTrafficEstimationRequest {
            targets,
            ..Default::default()
        }
    }
}

/// Request body for `historical_rank_overview`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_rank_overview/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsHistoricalRankOverviewRequest {
    /// Domain, subdomain, or webpage the request targets.
    pub target: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Start date of the range, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End date of the range, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Whether to align metrics across the returned dates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlate: Option<bool>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsHistoricalRankOverviewRequest {
    /// Creates a request for the given target domain.
    pub fn new(target: &str) -> DataForSeoLabsHistoricalRankOverviewRequest {
        DataForSeoLabsHistoricalRankOverviewRequest {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `categories_for_domain`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/categories_for_domain/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsCategoriesForDomainRequest {
    /// Domain, subdomain, or webpage the request targets.
    pub target: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether to include subcategories in the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subcategories: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// How to treat historical SERP data: `"live"`, `"lost"`, or `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_serp_mode: Option<String>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsCategoriesForDomainRequest {
    /// Creates a request for the given target domain.
    pub fn new(target: &str) -> DataForSeoLabsCategoriesForDomainRequest {
        DataForSeoLabsCategoriesForDomainRequest {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `categories_for_keywords`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/categories_for_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsCategoriesForKeywordsRequest {
    /// Keywords to analyze.
    pub keywords: Vec<String>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsCategoriesForKeywordsRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> DataForSeoLabsCategoriesForKeywordsRequest {
        DataForSeoLabsCategoriesForKeywordsRequest {
            keywords,
            ..Default::default()
        }
    }
}

/// Request body for `keywords_for_categories`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_categories/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordsForCategoriesRequest {
    /// Product-category codes to query.
    pub category_codes: Vec<String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Whether returned keywords must belong to all given categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_intersection: Option<bool>,
    /// Whether to include SERP metadata for each keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_serp_info: Option<bool>,
    /// Whether to enrich the response with clickstream-based metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_clickstream_data: Option<bool>,
    /// Whether to exclude highly similar keywords from the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_synonyms: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Token used to page beyond the standard offset limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsKeywordsForCategoriesRequest {
    /// Creates a request for the given product-category codes.
    pub fn new(category_codes: Vec<String>) -> DataForSeoLabsKeywordsForCategoriesRequest {
        DataForSeoLabsKeywordsForCategoriesRequest {
            category_codes,
            ..Default::default()
        }
    }
}

/// Request body for `domain_metrics_by_categories`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_metrics_by_categories/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsDomainMetricsByCategoriesRequest {
    /// Product-category codes to query.
    pub category_codes: Vec<String>,
    /// First date to compare, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_date: Option<String>,
    /// Second date to compare, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_date: Option<String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// Number of top categories to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_categories_count: Option<i64>,
    /// Whether to include subdomains of the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Minimum estimated-traffic-volume filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_min: Option<i64>,
    /// Maximum estimated-traffic-volume filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_max: Option<i64>,
    /// Whether to align metrics across the returned dates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlate: Option<bool>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsDomainMetricsByCategoriesRequest {
    /// Creates a request for the given product-category codes.
    pub fn new(category_codes: Vec<String>) -> DataForSeoLabsDomainMetricsByCategoriesRequest {
        DataForSeoLabsDomainMetricsByCategoriesRequest {
            category_codes,
            ..Default::default()
        }
    }
}

/// Request body for `keywords_for_app`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_app/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordsForAppRequest {
    /// Store identifier of the app.
    pub app_id: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsKeywordsForAppRequest {
    /// Creates a request for the given Google Play app id.
    pub fn new(app_id: &str) -> DataForSeoLabsKeywordsForAppRequest {
        DataForSeoLabsKeywordsForAppRequest {
            app_id: app_id.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `app_competitors`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/app_competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppCompetitorsRequest {
    /// Store identifier of the app.
    pub app_id: String,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsAppCompetitorsRequest {
    /// Creates a request for the given Google Play app id.
    pub fn new(app_id: &str) -> DataForSeoLabsAppCompetitorsRequest {
        DataForSeoLabsAppCompetitorsRequest {
            app_id: app_id.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for `app_intersection`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/app_intersection/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppIntersectionRequest {
    /// App identifiers, keyed by an index string.
    pub app_ids: HashMap<String, String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Array of filtering conditions applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsAppIntersectionRequest {
    /// Creates a request over the given app ids map (index string -> app id).
    pub fn new(app_ids: HashMap<String, String>) -> DataForSeoLabsAppIntersectionRequest {
        DataForSeoLabsAppIntersectionRequest {
            app_ids,
            ..Default::default()
        }
    }
}

/// Request body for `bulk_app_metrics`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/bulk_app_metrics/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsBulkAppMetricsRequest {
    /// App identifiers, keyed by an index string.
    pub app_ids: Vec<String>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DataForSeoLabsBulkAppMetricsRequest {
    /// Creates a request for the given Google Play app ids.
    pub fn new(app_ids: Vec<String>) -> DataForSeoLabsBulkAppMetricsRequest {
        DataForSeoLabsBulkAppMetricsRequest {
            app_ids,
            ..Default::default()
        }
    }
}
