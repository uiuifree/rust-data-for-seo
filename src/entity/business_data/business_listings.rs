//! Entities for the Business Listings endpoints.
//! See <https://docs.dataforseo.com/v3/business_data/business_listings/overview/>.

use crate::entity::BusinessDataApiBusinessItem;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request body for the Business Listings `search/live` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/business_listings/search/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiBusinessListingsSearchRequest {
    /// Business categories to filter by (up to 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// Business description to match (max 200 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Business name to match (max 200 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Restrict to businesses claimed (verified) on Google Maps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_claimed: Option<bool>,
    /// GPS coordinates as `"latitude,longitude,radius_km"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Result filtering conditions (up to 8; raw filter expressions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules applied to the results (up to 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Number of results to skip (default `0`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Token from a previous response used to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_token: Option<String>,
    /// Maximum number of results to return (default `100`, max `1000`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BusinessDataApiBusinessListingsSearchRequest {
    /// Builds an empty search request; set at least one filter before sending.
    pub fn new() -> Self {
        BusinessDataApiBusinessListingsSearchRequest::default()
    }
}

/// Request body for the Business Listings `categories_aggregation/live` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/business_listings/categories_aggregation/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiCategoriesAggregationRequest {
    /// Business categories to filter by (up to 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// Business description to match (max 200 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Business name to match (max 200 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Restrict to businesses claimed (verified) on Google Maps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_claimed: Option<bool>,
    /// GPS coordinates as `"latitude,longitude,radius_km"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Filters applied to the source dataset before aggregation (up to 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_dataset_filters: Option<Vec<Value>>,
    /// Maximum entries per aggregated category list (default `10`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// Number of results to skip (default `0`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of businesses to aggregate (default `100`, max `1000`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BusinessDataApiCategoriesAggregationRequest {
    /// Builds an empty categories aggregation request.
    pub fn new() -> Self {
        BusinessDataApiCategoriesAggregationRequest::default()
    }
}

/// A location entry returned by the Business Listings `locations` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/business_listings/locations/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiBusinessListingsLocation {
    /// Full location name (e.g. a country or region name).
    pub location_name: Option<String>,
    /// Two-letter ISO country code of the location.
    pub country_iso_code: Option<String>,
    /// Number of businesses in this location in the database.
    pub business_count: Option<i64>,
}

/// A category entry returned by the Business Listings `categories` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/business_listings/categories/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiBusinessListingsCategory {
    /// Category name/identifier (e.g. `"restaurant"`).
    pub category_name: Option<String>,
    /// Number of businesses classified under this category.
    pub business_count: Option<i64>,
}

/// Result of a Business Listings search: pagination metadata plus listings.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiBusinessListingsSearchResult {
    /// Total number of results matching the query in the database.
    pub total_count: Option<i64>,
    /// Number of results returned in this response.
    pub count: Option<i64>,
    /// Offset applied to this response.
    pub offset: Option<i64>,
    /// Token to pass in the next request to fetch the following page.
    pub offset_token: Option<String>,
    /// Number of elements in `items`.
    pub items_count: Option<i64>,
    /// Parsed business listings.
    pub items: Option<Vec<BusinessDataApiBusinessItem>>,
}

impl BusinessDataApiBusinessListingsSearchResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiBusinessItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// Result of a categories aggregation request: pagination metadata plus aggregates.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiCategoriesAggregationResult {
    /// Total number of results matching the query in the database.
    pub total_count: Option<i64>,
    /// Number of results returned in this response.
    pub count: Option<i64>,
    /// Offset applied to this response.
    pub offset: Option<i64>,
    /// Token to pass in the next request to fetch the following page.
    pub offset_token: Option<String>,
    /// Number of elements in `items`.
    pub items_count: Option<i64>,
    /// Parsed category aggregation elements.
    pub items: Option<Vec<BusinessDataApiCategoryAggregationItem>>,
}

impl BusinessDataApiCategoriesAggregationResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiCategoryAggregationItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A single aggregated business category and its aggregated metrics.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiCategoryAggregationItem {
    /// Element type, e.g. `"business_category"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Category name this aggregation is for.
    pub category: Option<String>,
    /// Google My Business category classification path.
    pub categories: Option<Vec<String>>,
    /// Number of businesses in this category.
    pub count: Option<i64>,
    /// Aggregated metrics (top categories, countries, attributes; raw object).
    pub aggregation: Option<Value>,
}
