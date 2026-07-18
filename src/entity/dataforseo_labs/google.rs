//! Google-specific item models for the DataForSEO Labs API.

use crate::entity::{
    DataForSeoLabsKeywordData, DataForSeoLabsKeywordInfo, DataForSeoLabsMetrics,
    DataForSeoLabsMetricsInfo, DataForSeoLabsRankedSerpElement, DataForSeoLabsSerpItem,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// One keyword a target ranks for.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsRankedKeywordItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// The ranked SERP position the target holds for this keyword.
    pub ranked_serp_element: Option<DataForSeoLabsRankedSerpElement>,
    /// Keyword metrics for the returned keyword.
    pub keyword_data: Option<DataForSeoLabsKeywordData>,
}

/// One keyword related to a seed keyword, plus its expansion depth.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/related_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsRelatedKeywordItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword metrics for the returned keyword.
    pub keyword_data: Option<DataForSeoLabsKeywordData>,
    /// Keyword expansion depth (0-4); higher values return more keywords.
    pub depth: Option<i64>,
    /// Keywords related to this keyword.
    pub related_keywords: Option<Vec<String>>,
}

/// Search-intent classification of a single keyword.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/search_intent/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSearchIntentItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword the data relates to.
    pub keyword: Option<String>,
    /// Primary search-intent classification.
    pub keyword_intent: Option<DataForSeoLabsKeywordIntent>,
    /// Alternative search-intent classifications, by likelihood.
    pub secondary_keyword_intents: Option<Vec<DataForSeoLabsKeywordIntent>>,
}

/// A labelled intent with its probability.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/search_intent/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordIntent {
    /// Intent label: `"informational"`, `"navigational"`, `"commercial"`, or `"transactional"`.
    pub label: Option<String>,
    /// Confidence of the classification, from 0 to 1.
    pub probability: Option<f64>,
}

/// Keyword difficulty score for a single keyword.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/bulk_keyword_difficulty/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordDifficultyItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword the data relates to.
    pub keyword: Option<String>,
    /// Difficulty of ranking in the top 10, from 0 to 100.
    pub keyword_difficulty: Option<i64>,
}

/// Historical monthly Google Ads metrics for a keyword.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_keyword_data/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsHistoricalKeywordDataItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword the data relates to.
    pub keyword: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    pub location_code: Option<i64>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    pub language_code: Option<String>,
    /// Metrics for each historical month.
    pub history: Option<Vec<DataForSeoLabsKeywordHistory>>,
}

/// One month of historical keyword metrics.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_keyword_data/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordHistory {
    /// Year the data point refers to.
    pub year: Option<i64>,
    /// Month the data point refers to (1-12).
    pub month: Option<i64>,
    /// Google Ads keyword metrics.
    pub keyword_info: Option<DataForSeoLabsKeywordInfo>,
}

/// A competing domain and its ranking metrics.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/competitors_domain/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsCompetitorItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Average SERP position across intersecting keywords.
    pub avg_position: Option<f64>,
    /// Sum of SERP positions across intersecting keywords.
    pub sum_position: Option<i64>,
    /// Number of keywords the targets have in common.
    pub intersections: Option<i64>,
    /// Metrics across all of the competitor's keywords.
    pub full_domain_metrics: Option<DataForSeoLabsMetrics>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<DataForSeoLabsMetrics>,
    /// Metrics limited to keywords shared with the target.
    pub competitor_metrics: Option<DataForSeoLabsMetrics>,
}

/// A domain competing for a set of keywords in the live SERP.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/serp_competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSerpCompetitorItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Average SERP position across intersecting keywords.
    pub avg_position: Option<f64>,
    /// Median SERP position across the keywords.
    pub median_position: Option<f64>,
    /// Visibility rating of the domain, from 0 to 1000.
    pub rating: Option<i64>,
    /// Estimated monthly traffic volume (CTR times search volume across ranking keywords).
    pub etv: Option<f64>,
    /// Number of keywords analyzed for the domain.
    pub keywords_count: Option<i64>,
    /// Share of possible SERP visibility captured, from 0 to 1.
    pub visibility: Option<f64>,
    /// Number of relevant SERP elements found for the domain.
    pub relevant_serp_items: Option<i64>,
    /// Positions the domain holds, keyed by keyword.
    pub keywords_positions: Option<HashMap<String, Vec<i64>>>,
}

/// Ranking metrics of a domain in one location/language.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_rank_overview/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsDomainMetricsItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    pub location_code: Option<i64>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    pub language_code: Option<String>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<DataForSeoLabsMetrics>,
}

/// Estimated traffic metrics for a single target.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/bulk_traffic_estimation/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsTrafficEstimationItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Domain, subdomain, or webpage the request targets.
    pub target: Option<String>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<DataForSeoLabsMetrics>,
}

/// Ranking metrics of a subdomain.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/subdomains/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSubdomainItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Subdomain the metrics belong to.
    pub subdomain: Option<String>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<DataForSeoLabsMetrics>,
}

/// Ranking metrics of a single page.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/relevant_pages/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsRelevantPageItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Absolute URL of the page.
    pub page_address: Option<String>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<DataForSeoLabsMetrics>,
}

/// A keyword both domains rank for, with each domain's SERP element.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_intersection/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsDomainIntersectionItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword metrics for the returned keyword.
    pub keyword_data: Option<DataForSeoLabsKeywordData>,
    /// SERP element the first domain holds for the keyword.
    pub first_domain_serp_element: Option<DataForSeoLabsSerpItem>,
    /// SERP element the second domain holds for the keyword.
    pub second_domain_serp_element: Option<DataForSeoLabsSerpItem>,
}

/// A keyword with the SERP element each analyzed page holds for it.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/page_intersection/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsPageIntersectionItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword metrics for the returned keyword.
    pub keyword_data: Option<DataForSeoLabsKeywordData>,
    /// SERP element each analyzed target holds for the keyword, keyed by target.
    pub intersection_result: Option<HashMap<String, DataForSeoLabsSerpItem>>,
}

/// Ranking metrics of a domain for one historical month.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_rank_overview/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsHistoricalRankOverviewItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Year the data point refers to.
    pub year: Option<i64>,
    /// Month the data point refers to (1-12).
    pub month: Option<i64>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<DataForSeoLabsMetrics>,
}

/// A historical SERP snapshot for a keyword.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_serps/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsHistoricalSerpItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword the data relates to.
    pub keyword: Option<String>,
    /// Type of SERP element (serialized as `type`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Search-engine domain the SERP was collected from (e.g. `"google.com"`).
    pub se_domain: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    pub location_code: Option<i64>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    pub language_code: Option<String>,
    /// URL used to check the SERP.
    pub check_url: Option<String>,
    /// Date and time of the record (UTC).
    pub datetime: Option<String>,
    /// Spelling correction applied by the search engine, if any.
    pub spell: Option<Value>,
    /// SERP feature types to include (e.g. `"organic"`, `"paid"`).
    pub item_types: Option<Vec<String>>,
    /// Total number of results the search engine reported.
    pub se_results_count: Option<i64>,
    /// Number of items returned in `items`.
    pub items_count: Option<i64>,
    /// Result items.
    pub items: Option<Vec<DataForSeoLabsSerpItem>>,
}

/// Ranking metrics of a domain for one product category.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/categories_for_domain/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsCategoryMetricsItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Product and service category codes for the keyword.
    pub categories: Option<Vec<i64>>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<DataForSeoLabsMetrics>,
}

/// Estimated traffic of a target for one historical month.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_bulk_traffic_estimation/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsHistoricalMetricsInfo {
    /// Year the data point refers to.
    pub year: Option<i64>,
    /// Month the data point refers to (1-12).
    pub month: Option<i64>,
    /// Estimated monthly traffic volume (CTR times search volume across ranking keywords).
    pub etv: Option<f64>,
    /// Number of SERPs containing the target for this feature type.
    pub count: Option<i64>,
}

/// Historical traffic metrics grouped by SERP feature type.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_bulk_traffic_estimation/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsHistoricalMetricsBundle {
    /// Metrics for organic search results.
    pub organic: Option<Vec<DataForSeoLabsHistoricalMetricsInfo>>,
    /// Metrics for paid search results.
    pub paid: Option<Vec<DataForSeoLabsHistoricalMetricsInfo>>,
    /// Metrics for local pack results.
    pub local_pack: Option<Vec<DataForSeoLabsHistoricalMetricsInfo>>,
    /// Metrics for featured snippet results.
    pub featured_snippet: Option<Vec<DataForSeoLabsHistoricalMetricsInfo>>,
}

/// Historical estimated traffic for a single target.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/historical_bulk_traffic_estimation/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsHistoricalTrafficEstimationItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Domain, subdomain, or webpage the request targets.
    pub target: Option<String>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<DataForSeoLabsHistoricalMetricsBundle>,
}

/// Product categories associated with a keyword.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/categories_for_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsCategoriesForKeywordsItem {
    /// Keyword the data relates to.
    pub keyword: Option<String>,
    /// Product and service category codes for the keyword.
    pub categories: Option<Vec<i64>>,
}

/// Ranking metrics of a domain collected for a set of product categories.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_metrics_by_categories/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsDomainMetricsByCategoriesItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Category codes the domains were collected for.
    pub top_categories: Option<Vec<i64>>,
    /// Current organic estimated traffic volume of the domain.
    pub organic_etv: Option<f64>,
    /// Current number of organic SERPs containing the domain.
    pub organic_count: Option<i64>,
    /// Current number of lost organic ranked elements.
    pub organic_is_lost: Option<i64>,
    /// Current number of new organic ranked elements.
    pub organic_is_new: Option<i64>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Primary (registrable) domain.
    pub main_domain: Option<String>,
    /// Historical ranking metrics keyed by date and SERP feature type.
    pub metrics_history: Option<HashMap<String, HashMap<String, DataForSeoLabsMetricsInfo>>>,
    /// Change in metrics between the two compared dates, by SERP feature type.
    pub metrics_difference: Option<HashMap<String, DataForSeoLabsMetricsInfo>>,
}
