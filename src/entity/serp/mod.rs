mod baidu;
mod bing;
mod element;
mod google;
mod languages;
mod naver;
mod seznam;
mod yahoo;
mod youtube;

pub use baidu::*;
pub use bing::*;
pub use element::*;
pub use google::*;
pub use languages::*;
pub use naver::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use seznam::*;
pub use yahoo::*;
pub use youtube::*;

/// Html Item SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiHtmlItem {
    /// Result page number.
    pub page: i32,
    /// Date associated with the result.
    pub date: String,
    /// Raw HTML of the page.
    pub html: String,
}

/// Null-safe SERP result wrapper: search metadata plus the parsed `items` array.
///
/// Every field is optional so results from engines that omit some metadata
/// (e.g. location-less engines like Naver) deserialize without error.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiSearchResult<T> {
    /// Search term the result was returned for.
    pub keyword: Option<String>,
    /// Search engine result type (the API `type` field).
    #[serde(rename = "type")]
    pub search_engine_type: Option<String>,
    /// Search-engine domain the results were taken from.
    pub se_domain: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: Option<i32>,
    /// Language code the search was run for.
    pub language_code: Option<String>,
    /// Direct URL to reproduce the search on the search engine.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was received.
    pub datetime: Option<String>,
    /// Search-engine spelling correction applied to the query, if any.
    pub spell: Option<Value>,
    /// Search-refinement chips shown for the query.
    pub refinement_chips: Option<SerpApiElementRefinementChips>,
    /// Distinct element types present in the returned SERP.
    pub item_types: Option<Vec<String>>,
    /// Total number of results reported by the search engine.
    pub se_results_count: Option<i64>,
    /// Number of items returned in this result.
    pub items_count: Option<i64>,
    /// Parsed elements of the result.
    pub items: Option<Vec<T>>,
}

impl<T> SerpApiSearchResult<T> {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[T] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// Rating SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiRating {
    /// Scale of the rating (e.g. `Max5`).
    pub rating_type: Option<String>,
    /// Numeric value.
    pub value: Option<f32>,
    /// Number of votes the rating is based on.
    pub votes_count: Option<u32>,
    /// Maximum possible rating value.
    pub rating_max: Option<u32>,
}
/// Rating Distribution SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiRatingDistribution {
    /// Number of 1-star ratings.
    #[serde(rename = "1")]
    pub distribution1: Option<i32>,
    /// Number of 2-star ratings.
    #[serde(rename = "2")]
    pub distribution2: Option<i32>,
    /// Number of 3-star ratings.
    #[serde(rename = "3")]
    pub distribution3: Option<i32>,
    /// Number of 4-star ratings.
    #[serde(rename = "4")]
    pub distribution4: Option<i32>,
    /// Number of 5-star ratings.
    #[serde(rename = "5")]
    pub distribution5: Option<i32>,
}
/// Rectangle SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiRectangle {
    /// X coordinate of the bounding box, in pixels.
    pub x: Option<f32>,
    /// Y coordinate of the bounding box, in pixels.
    pub y: Option<f32>,
    /// Width of the bounding box, in pixels.
    pub width: Option<f32>,
    /// Height of the bounding box, in pixels.
    pub height: Option<f32>,
}

/// Price SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiPrice {
    /// Current price.
    pub current: Option<f32>,
    /// Regular (non-discounted) price.
    pub regular: Option<f32>,
    /// Upper bound of the price range.
    pub max_value: Option<f32>,
    /// ISO currency code of the price.
    pub currency: Option<String>,
    /// `true` if the price represents a range.
    pub is_price_range: Option<bool>,
    /// Price string as displayed in the SERP.
    pub displayed_price: Option<String>,
}
