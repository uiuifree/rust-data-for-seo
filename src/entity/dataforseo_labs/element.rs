//! Shared building blocks for DataForSEO Labs response models.
//!
//! These structs are reused across many Labs endpoints (ranked_keywords,
//! keyword_ideas, competitors_domain, ...).
//! See <https://docs.dataforseo.com/v3/dataforseo_labs/overview/>.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Google Ads keyword metrics attached to a keyword.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_ideas/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordInfo {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
    /// Google Ads competition level, from 0 to 1.
    pub competition: Option<f64>,
    /// Competition category: `"LOW"`, `"MEDIUM"`, or `"HIGH"`.
    pub competition_level: Option<String>,
    /// Average cost per click (USD).
    pub cpc: Option<f64>,
    /// Average monthly search volume.
    pub search_volume: Option<i64>,
    /// Low range of the top-of-page bid (USD).
    pub low_top_of_page_bid: Option<f64>,
    /// High range of the top-of-page bid (USD).
    pub high_top_of_page_bid: Option<f64>,
    /// Product and service category codes for the keyword.
    pub categories: Option<Vec<i64>>,
    /// Search volume for each of the preceding months.
    pub monthly_searches: Option<Vec<DataForSeoLabsMonthlySearch>>,
    /// Percentage change of search volume across recent periods.
    pub search_volume_trend: Option<DataForSeoLabsSearchVolumeTrend>,
}

/// A single month of historical search volume.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsMonthlySearch {
    /// Year the data point refers to.
    pub year: Option<i64>,
    /// Month the data point refers to (1-12).
    pub month: Option<i64>,
    /// Average monthly search volume.
    pub search_volume: Option<i64>,
}

/// Percentage change of search volume across recent periods.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSearchVolumeTrend {
    /// Percentage change compared with the previous month.
    pub monthly: Option<i64>,
    /// Percentage change compared with the previous quarter.
    pub quarterly: Option<i64>,
    /// Percentage change compared with the previous year.
    pub yearly: Option<i64>,
}

/// Search volume normalized with a secondary data source (Bing or clickstream).
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_ideas/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordInfoNormalized {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
    /// Whether the search volume was normalized with the secondary source.
    pub is_normalized: Option<bool>,
    /// Average monthly search volume.
    pub search_volume: Option<i64>,
    /// Search volume for each of the preceding months.
    pub monthly_searches: Option<Vec<DataForSeoLabsMonthlySearch>>,
}

/// Clickstream-based keyword metrics.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_ideas/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsClickstreamKeywordInfo {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
    /// Average monthly search volume.
    pub search_volume: Option<i64>,
    /// Clickstream traffic split by gender.
    pub gender_distribution: Option<DataForSeoLabsGenderDistribution>,
    /// Clickstream traffic split by age bracket.
    pub age_distribution: Option<DataForSeoLabsAgeDistribution>,
    /// Search volume for each of the preceding months.
    pub monthly_searches: Option<Vec<DataForSeoLabsMonthlySearch>>,
}

/// Structural properties of a keyword (difficulty, language, clustering).
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_ideas/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordProperties {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Canonical keyword this keyword was clustered under.
    pub core_keyword: Option<String>,
    /// Algorithm used to cluster synonymous keywords.
    pub synonym_clustering_algorithm: Option<String>,
    /// Difficulty of ranking in the top 10, from 0 to 100.
    pub keyword_difficulty: Option<i64>,
    /// Language detected for the keyword.
    pub detected_language: Option<String>,
    /// Whether the keyword is in a language other than requested.
    pub is_another_language: Option<bool>,
    /// Number of words in the keyword.
    pub words_count: Option<i64>,
}

/// Estimated Google Ads impression/click metrics for a keyword.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsImpressionsInfo {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
    /// Bid value the impression estimates are based on (USD).
    pub bid: Option<f64>,
    /// Keyword match type the estimates are based on.
    pub match_type: Option<String>,
    /// Minimum estimated ad position.
    pub ad_position_min: Option<f64>,
    /// Maximum estimated ad position.
    pub ad_position_max: Option<f64>,
    /// Average estimated ad position.
    pub ad_position_average: Option<f64>,
    /// Minimum estimated cost per click (USD).
    pub cpc_min: Option<f64>,
    /// Maximum estimated cost per click (USD).
    pub cpc_max: Option<f64>,
    /// Average estimated cost per click (USD).
    pub cpc_average: Option<f64>,
    /// Minimum estimated daily impressions.
    pub daily_impressions_min: Option<f64>,
    /// Maximum estimated daily impressions.
    pub daily_impressions_max: Option<f64>,
    /// Average estimated daily impressions.
    pub daily_impressions_average: Option<f64>,
    /// Minimum estimated daily clicks.
    pub daily_clicks_min: Option<f64>,
    /// Maximum estimated daily clicks.
    pub daily_clicks_max: Option<f64>,
    /// Average estimated daily clicks.
    pub daily_clicks_average: Option<f64>,
    /// Minimum estimated daily ad cost (USD).
    pub daily_cost_min: Option<f64>,
    /// Maximum estimated daily ad cost (USD).
    pub daily_cost_max: Option<f64>,
    /// Average estimated daily ad cost (USD).
    pub daily_cost_average: Option<f64>,
}

/// Metadata about the SERP a keyword was last checked against.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_ideas/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSerpInfo {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// URL used to check the SERP.
    pub check_url: Option<String>,
    /// Types of SERP elements present in the results.
    pub serp_item_types: Option<Vec<String>>,
    /// Total number of results the search engine reported.
    pub se_results_count: Option<i64>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
    /// Timestamp of the previous data update (UTC).
    pub previous_updated_time: Option<String>,
}

/// Average backlink metrics of the pages ranking for a keyword.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_ideas/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAvgBacklinksInfo {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Average backlinks of the ranking pages.
    pub backlinks: Option<f64>,
    /// Average dofollow backlinks of the ranking pages.
    pub dofollow: Option<f64>,
    /// Average referring pages of the ranking pages.
    pub referring_pages: Option<f64>,
    /// Average referring domains of the ranking pages.
    pub referring_domains: Option<f64>,
    /// Average referring main domains of the ranking pages.
    pub referring_main_domains: Option<f64>,
    /// Average rank of the ranking pages.
    pub rank: Option<f64>,
    /// Average main-domain rank of the ranking pages.
    pub main_domain_rank: Option<f64>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
}

/// Search intent classification attached to a keyword.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_ideas/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSearchIntentInfo {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Dominant search intent of the keyword.
    pub main_intent: Option<String>,
    /// Additional detected search intents.
    pub foreign_intent: Option<Vec<String>>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
}

/// Full set of metrics DataForSEO stores for a single keyword.
///
/// Used both as a standalone item (keyword_ideas, keyword_suggestions,
/// keywords_for_site, keyword_overview, top_searches) and nested inside
/// ranked/related keyword items.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keyword_ideas/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsKeywordData {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword the data relates to.
    pub keyword: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    pub location_code: Option<i32>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    pub language_code: Option<String>,
    /// Google Ads keyword metrics.
    pub keyword_info: Option<DataForSeoLabsKeywordInfo>,
    /// Search volume normalized with Bing data.
    pub keyword_info_normalized_with_bing: Option<DataForSeoLabsKeywordInfoNormalized>,
    /// Search volume normalized with clickstream data.
    pub keyword_info_normalized_with_clickstream: Option<DataForSeoLabsKeywordInfoNormalized>,
    /// Clickstream-based keyword metrics.
    pub clickstream_keyword_info: Option<DataForSeoLabsClickstreamKeywordInfo>,
    /// Structural properties of the keyword.
    pub keyword_properties: Option<DataForSeoLabsKeywordProperties>,
    /// Estimated Google Ads impression and click metrics.
    pub impressions_info: Option<DataForSeoLabsImpressionsInfo>,
    /// Metadata about the SERP the keyword was last checked against.
    pub serp_info: Option<DataForSeoLabsSerpInfo>,
    /// Average backlink metrics of the ranking pages.
    pub avg_backlinks_info: Option<DataForSeoLabsAvgBacklinksInfo>,
    /// Search-intent classification of the keyword.
    pub search_intent_info: Option<DataForSeoLabsSearchIntentInfo>,
}

/// Clickstream gender split.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsGenderDistribution {
    /// Estimated share of female users.
    pub female: Option<i64>,
    /// Estimated share of male users.
    pub male: Option<i64>,
}

/// Clickstream age split.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAgeDistribution {
    /// Estimated traffic share for the 18-24 age bracket.
    #[serde(rename = "18-24")]
    pub age_18_24: Option<i64>,
    /// Estimated traffic share for the 25-34 age bracket.
    #[serde(rename = "25-34")]
    pub age_25_34: Option<i64>,
    /// Estimated traffic share for the 35-44 age bracket.
    #[serde(rename = "35-44")]
    pub age_35_44: Option<i64>,
    /// Estimated traffic share for the 45-54 age bracket.
    #[serde(rename = "45-54")]
    pub age_45_54: Option<i64>,
    /// Estimated traffic share for the 55-64 age bracket.
    #[serde(rename = "55-64")]
    pub age_55_64: Option<i64>,
}

/// Aggregated ranking metrics grouped by SERP feature type.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_rank_overview/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsMetrics {
    /// Metrics for organic search results.
    pub organic: Option<DataForSeoLabsMetricsInfo>,
    /// Metrics for paid search results.
    pub paid: Option<DataForSeoLabsMetricsInfo>,
    /// Metrics for featured snippet results.
    pub featured_snippet: Option<DataForSeoLabsMetricsInfo>,
    /// Metrics for local pack results.
    pub local_pack: Option<DataForSeoLabsMetricsInfo>,
}

/// Ranking metrics for one SERP feature type (organic, paid, ...).
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/domain_rank_overview/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsMetricsInfo {
    /// Number of SERPs where the target ranks #1.
    pub pos_1: Option<i64>,
    /// Number of SERPs where the target ranks #2-3.
    pub pos_2_3: Option<i64>,
    /// Number of SERPs where the target ranks #4-10.
    pub pos_4_10: Option<i64>,
    /// Number of SERPs where the target ranks #11-20.
    pub pos_11_20: Option<i64>,
    /// Number of SERPs where the target ranks #21-30.
    pub pos_21_30: Option<i64>,
    /// Number of SERPs where the target ranks #31-40.
    pub pos_31_40: Option<i64>,
    /// Number of SERPs where the target ranks #41-50.
    pub pos_41_50: Option<i64>,
    /// Number of SERPs where the target ranks #51-60.
    pub pos_51_60: Option<i64>,
    /// Number of SERPs where the target ranks #61-70.
    pub pos_61_70: Option<i64>,
    /// Number of SERPs where the target ranks #71-80.
    pub pos_71_80: Option<i64>,
    /// Number of SERPs where the target ranks #81-90.
    pub pos_81_90: Option<i64>,
    /// Number of SERPs where the target ranks #91-100.
    pub pos_91_100: Option<i64>,
    /// Estimated monthly traffic volume (CTR times search volume across ranking keywords).
    pub etv: Option<f64>,
    /// Estimated traffic volume based on impressions.
    pub impressions_etv: Option<f64>,
    /// Number of SERPs containing the target for this feature type.
    pub count: Option<i64>,
    /// Estimated monthly cost (USD) to obtain this traffic via paid search.
    pub estimated_paid_traffic_cost: Option<f64>,
    /// Number of newly ranked elements.
    pub is_new: Option<i64>,
    /// Number of elements that improved in rank.
    pub is_up: Option<i64>,
    /// Number of elements that declined in rank.
    pub is_down: Option<i64>,
    /// Number of previously ranked elements no longer found.
    pub is_lost: Option<i64>,
    /// Estimated traffic volume derived from clickstream data.
    pub clickstream_etv: Option<f64>,
    /// Clickstream traffic split by gender.
    pub clickstream_gender_distribution: Option<DataForSeoLabsGenderDistribution>,
    /// Clickstream traffic split by age bracket.
    pub clickstream_age_distribution: Option<DataForSeoLabsAgeDistribution>,
}

/// A ranked SERP position held by a target, wrapping the SERP item.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsRankedSerpElement {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// The SERP element occupying this ranked position.
    pub serp_item: Option<DataForSeoLabsSerpItem>,
    /// URL used to check the SERP.
    pub check_url: Option<String>,
    /// Types of SERP elements present in the results.
    pub serp_item_types: Option<Vec<String>>,
    /// Total number of results the search engine reported.
    pub se_results_count: Option<i64>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
    /// Timestamp of the previous data update (UTC).
    pub previous_updated_time: Option<String>,
}

/// A single SERP element (organic result, paid result, featured snippet, ...).
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSerpItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Type of SERP element (serialized as `type`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank among all SERP elements.
    pub rank_absolute: Option<i64>,
    /// Placement within the SERP (e.g. `"left"` or `"right"`).
    pub position: Option<String>,
    /// XPath of the element in the SERP HTML.
    pub xpath: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result (for an id-list entry, the endpoint the task was posted to).
    pub url: Option<String>,
    /// URL path relative to the domain.
    pub relative_url: Option<String>,
    /// Text snippet of the result.
    pub description: Option<String>,
    /// Estimated monthly traffic volume (CTR times search volume across ranking keywords).
    pub etv: Option<f64>,
    /// Estimated traffic volume based on impressions.
    pub impressions_etv: Option<f64>,
    /// Estimated monthly cost (USD) to obtain this traffic via paid search.
    pub estimated_paid_traffic_cost: Option<f64>,
    /// Estimated traffic volume derived from clickstream data.
    pub clickstream_etv: Option<f64>,
    /// How the element's rank changed since the previous check.
    pub rank_changes: Option<DataForSeoLabsRankChanges>,
    /// Backlink counters of the ranking page.
    pub backlinks_info: Option<DataForSeoLabsSerpBacklinksInfo>,
    /// Page and domain rank of the ranking page.
    pub rank_info: Option<DataForSeoLabsRankInfo>,
}

/// Position movement of a SERP element between the last two checks.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsRankChanges {
    /// Absolute rank at the previous check.
    pub previous_rank_absolute: Option<i64>,
    /// Whether the element is newly ranked.
    pub is_new: Option<bool>,
    /// Whether the element moved up in rank.
    pub is_up: Option<bool>,
    /// Whether the element moved down in rank.
    pub is_down: Option<bool>,
}

/// Backlink counters of a ranking page.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsSerpBacklinksInfo {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Number of referring domains pointing to the page.
    pub referring_domains: Option<i64>,
    /// Number of referring main domains pointing to the page.
    pub referring_main_domains: Option<i64>,
    /// Number of referring pages pointing to the page.
    pub referring_pages: Option<i64>,
    /// Number of dofollow backlinks pointing to the page.
    pub dofollow: Option<i64>,
    /// Number of backlinks pointing to the page.
    pub backlinks: Option<i64>,
    /// Timestamp when the backlink data was updated (UTC).
    pub time_update: Option<String>,
}

/// Page/domain rank of a ranking page.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsRankInfo {
    /// Rank of the page, from 0 to 1000.
    pub page_rank: Option<i64>,
    /// Rank of the page's main domain, from 0 to 1000.
    pub main_domain_rank: Option<i64>,
}

/// Common result wrapper shared by DataForSEO Labs endpoints.
///
/// Individual endpoints populate the relevant metadata fields (a target, a
/// keyword, a seed keyword, ...) plus their `items` array. Fields that a
/// given endpoint does not return stay `None`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/overview/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsResult<T> {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Domain, subdomain, or webpage the request targets.
    pub target: Option<String>,
    /// First domain to compare.
    pub target1: Option<String>,
    /// Second domain to compare.
    pub target2: Option<String>,
    /// Store identifier of the app.
    pub app_id: Option<String>,
    /// App identifiers, keyed by an index string.
    pub app_ids: Option<HashMap<String, String>>,
    /// Keyword the data relates to.
    pub keyword: Option<String>,
    /// Seed keyword the results were expanded from.
    pub seed_keyword: Option<String>,
    /// Seed keywords the results were expanded from.
    pub seed_keywords: Option<Vec<String>>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    pub location_code: Option<i32>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    pub language_code: Option<String>,
    /// Total number of matching results in the database.
    pub total_count: Option<i64>,
    /// Number of items returned in `items`.
    pub items_count: Option<i64>,
    /// Offset into the result set, for pagination.
    pub offset: Option<i32>,
    /// Token used to page beyond the standard offset limit.
    pub offset_token: Option<String>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<DataForSeoLabsMetrics>,
    /// Absolute ranking and traffic metrics, grouped by SERP feature type.
    pub metrics_absolute: Option<DataForSeoLabsMetrics>,
    /// Result items.
    pub items: Option<Vec<T>>,
}

impl<T> DataForSeoLabsResult<T> {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[T] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// Raw JSON alias for endpoints whose item schema is not modeled in detail.
pub type DataForSeoLabsRawItem = Value;
