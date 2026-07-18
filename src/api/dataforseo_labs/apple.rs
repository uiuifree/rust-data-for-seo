//! Apple App Store endpoints of the DataForSEO Labs API.
//! See <https://docs.dataforseo.com/v3/dataforseo_labs/apple/>.

use crate::entity::{
    DataForSeoLabsAppCompetitorItem, DataForSeoLabsAppIntersectionItem,
    DataForSeoLabsAppKeywordItem, DataForSeoLabsBulkAppMetricsItem, DataForSeoLabsResult,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Apple App Store endpoints of the DataForSEO Labs API.
pub struct DataForSeoLabsApiApple<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl DataForSeoLabsApiApple<'_> {
    /// Keywords an App Store app ranks for.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/apple/keywords_for_app/live/>.
    pub async fn keywords_for_app(
        &self,
        data: Vec<DataForSeoLabsAppleKeywordsForAppRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAppKeywordItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/apple/keywords_for_app/live", &data)
            .await
    }

    /// App Store apps competing with a target app.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/apple/app_competitors/live/>.
    pub async fn app_competitors(
        &self,
        data: Vec<DataForSeoLabsAppleAppCompetitorsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAppCompetitorItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/apple/app_competitors/live", &data)
            .await
    }

    /// Keywords a set of App Store apps rank for together.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/apple/app_intersection/live/>.
    pub async fn app_intersection(
        &self,
        data: Vec<DataForSeoLabsAppleAppIntersectionRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsAppIntersectionItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/apple/app_intersection/live", &data)
            .await
    }

    /// Aggregated metrics for a batch of App Store apps.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/apple/bulk_app_metrics/live/>.
    pub async fn bulk_app_metrics(
        &self,
        data: Vec<DataForSeoLabsAppleBulkAppMetricsRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsResult<DataForSeoLabsBulkAppMetricsItem>> {
        self.client
            .http_post("/v3/dataforseo_labs/apple/bulk_app_metrics/live", &data)
            .await
    }
}

/// Request body for Apple `keywords_for_app`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/apple/keywords_for_app/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppleKeywordsForAppRequest {
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

impl DataForSeoLabsAppleKeywordsForAppRequest {
    /// Creates a request for the given App Store app id.
    pub fn new(app_id: &str) -> DataForSeoLabsAppleKeywordsForAppRequest {
        DataForSeoLabsAppleKeywordsForAppRequest {
            app_id: app_id.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for Apple `app_competitors`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/apple/app_competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppleAppCompetitorsRequest {
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

impl DataForSeoLabsAppleAppCompetitorsRequest {
    /// Creates a request for the given App Store app id.
    pub fn new(app_id: &str) -> DataForSeoLabsAppleAppCompetitorsRequest {
        DataForSeoLabsAppleAppCompetitorsRequest {
            app_id: app_id.to_string(),
            ..Default::default()
        }
    }
}

/// Request body for Apple `app_intersection`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/apple/app_intersection/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppleAppIntersectionRequest {
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

impl DataForSeoLabsAppleAppIntersectionRequest {
    /// Creates a request over the given app ids map (index string -> app id).
    pub fn new(app_ids: HashMap<String, String>) -> DataForSeoLabsAppleAppIntersectionRequest {
        DataForSeoLabsAppleAppIntersectionRequest {
            app_ids,
            ..Default::default()
        }
    }
}

/// Request body for Apple `bulk_app_metrics`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/apple/bulk_app_metrics/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppleBulkAppMetricsRequest {
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

impl DataForSeoLabsAppleBulkAppMetricsRequest {
    /// Creates a request for the given App Store app ids.
    pub fn new(app_ids: Vec<String>) -> DataForSeoLabsAppleBulkAppMetricsRequest {
        DataForSeoLabsAppleBulkAppMetricsRequest {
            app_ids,
            ..Default::default()
        }
    }
}
