//! Amazon endpoints of the DataForSEO Labs API.
//! See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/>.

use crate::entity::{
    DataForSeoLabsAmazonProductCompetitorItem, DataForSeoLabsAmazonProductKeywordIntersectionItem,
    DataForSeoLabsAmazonProductRankOverviewItem, DataForSeoLabsAmazonRankedKeywordItem,
    DataForSeoLabsAmazonRelatedKeywordItem, DataForSeoLabsAmazonSearchVolumeItem,
    DataForSeoLabsResult,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Amazon endpoints of the DataForSEO Labs API.
pub struct DataForSeoLabsApiAmazon<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl DataForSeoLabsApiAmazon<'_> {
    /// Amazon search volume for a batch of keywords.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/bulk_search_volume/live/>.
    pub async fn bulk_search_volume(
        &self,
        data: Vec<DataForSeoLabsAmazonBulkSearchVolumeRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAmazonSearchVolumeItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/amazon/bulk_search_volume/live", &data)
            .await
    }

    /// Keywords related to a seed keyword on Amazon.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/related_keywords/live/>.
    pub async fn related_keywords(
        &self,
        data: Vec<DataForSeoLabsAmazonRelatedKeywordsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAmazonRelatedKeywordItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/amazon/related_keywords/live", &data)
            .await
    }

    /// Keywords an Amazon product (ASIN) ranks for.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/ranked_keywords/live/>.
    pub async fn ranked_keywords(
        &self,
        data: Vec<DataForSeoLabsAmazonRankedKeywordsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAmazonRankedKeywordItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/amazon/ranked_keywords/live", &data)
            .await
    }

    /// Aggregated ranking overview of an Amazon product.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_rank_overview/live/>.
    pub async fn product_rank_overview(
        &self,
        data: Vec<DataForSeoLabsAmazonProductRankOverviewRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAmazonProductRankOverviewItem>>
    {
        self.client
            .http_post(
                "/v3/dataforseo_labs/amazon/product_rank_overview/live",
                &data,
            )
            .await
    }

    /// Amazon products (ASINs) competing with a target product.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_competitors/live/>.
    pub async fn product_competitors(
        &self,
        data: Vec<DataForSeoLabsAmazonProductCompetitorsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAmazonProductCompetitorItem>>
    {
        self.client
            .http_post("/v3/dataforseo_labs/amazon/product_competitors/live", &data)
            .await
    }

    /// Keywords a set of Amazon products rank for together.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_keyword_intersections/live/>.
    pub async fn product_keyword_intersections(
        &self,
        data: Vec<DataForSeoLabsAmazonProductKeywordIntersectionsRequest>,
    ) -> DataForSeoApiResponse<
        DataForSeoLabsResult<DataForSeoLabsAmazonProductKeywordIntersectionItem>,
    > {
        self.client
            .http_post(
                "/v3/dataforseo_labs/amazon/product_keyword_intersections/live",
                &data,
            )
            .await
    }
}

/// Request body for Amazon `bulk_search_volume`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/bulk_search_volume/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonBulkSearchVolumeRequest {
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

impl DataForSeoLabsAmazonBulkSearchVolumeRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> DataForSeoLabsAmazonBulkSearchVolumeRequest {
        DataForSeoLabsAmazonBulkSearchVolumeRequest {
            keywords,
            ..Default::default()
        }
    }
}

/// Request body for Amazon `related_keywords`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/related_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonRelatedKeywordsRequest {
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

impl DataForSeoLabsAmazonRelatedKeywordsRequest {
    /// Creates a request for the given seed keyword.
    pub fn new(keyword: &str) -> DataForSeoLabsAmazonRelatedKeywordsRequest {
        DataForSeoLabsAmazonRelatedKeywordsRequest {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for Amazon `ranked_keywords`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonRankedKeywordsRequest {
    /// Amazon product identifier (ASIN).
    pub asin: String,
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

impl DataForSeoLabsAmazonRankedKeywordsRequest {
    /// Creates a request for the given product ASIN.
    pub fn new(asin: &str) -> DataForSeoLabsAmazonRankedKeywordsRequest {
        DataForSeoLabsAmazonRankedKeywordsRequest {
            asin: asin.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for Amazon `product_rank_overview`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_rank_overview/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonProductRankOverviewRequest {
    /// Amazon product identifier (ASIN).
    pub asin: String,
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

impl DataForSeoLabsAmazonProductRankOverviewRequest {
    /// Creates a request for the given product ASIN.
    pub fn new(asin: &str) -> DataForSeoLabsAmazonProductRankOverviewRequest {
        DataForSeoLabsAmazonProductRankOverviewRequest {
            asin: asin.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for Amazon `product_competitors`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonProductCompetitorsRequest {
    /// Amazon product identifier (ASIN).
    pub asin: String,
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

impl DataForSeoLabsAmazonProductCompetitorsRequest {
    /// Creates a request for the given product ASIN.
    pub fn new(asin: &str) -> DataForSeoLabsAmazonProductCompetitorsRequest {
        DataForSeoLabsAmazonProductCompetitorsRequest {
            asin: asin.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for Amazon `product_keyword_intersections`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_keyword_intersections/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonProductKeywordIntersectionsRequest {
    /// Product ASINs, keyed by an index string.
    pub asins: HashMap<String, String>,
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
    /// How the intersection is computed: `"union"` or `"intersect"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersection_mode: Option<String>,
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

impl DataForSeoLabsAmazonProductKeywordIntersectionsRequest {
    /// Creates a request over the given asins map (index string -> ASIN).
    pub fn new(
        asins: HashMap<String, String>,
    ) -> DataForSeoLabsAmazonProductKeywordIntersectionsRequest {
        DataForSeoLabsAmazonProductKeywordIntersectionsRequest {
            asins,
            ..Default::default()
        }
    }
}
