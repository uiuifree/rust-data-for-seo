use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A single DataForSEO Trends result, shared by the `explore`, `subregion_interests`,
/// `demography` and `merged_data` endpoints.
/// See <https://docs.dataforseo.com/v3/keywords_data/dataforseo_trends/explore/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsResult {
    /// Keywords the trends data was requested for.
    pub keywords: Option<Vec<String>>,
    /// Trends data source, e.g. "web", "news" or "ecommerce".
    #[serde(rename = "type")]
    pub trends_type: Option<String>,
    /// Location code the data was requested for.
    pub location_code: Option<i32>,
    /// Language code the data was requested for.
    pub language_code: Option<String>,
    /// UTC timestamp when the data was collected.
    pub datetime: Option<String>,
    /// Number of elements in `items`.
    pub items_count: Option<i32>,
    /// Trends elements returned for the request.
    pub items: Option<Vec<KeywordsDataApiDataforseoTrendsItem>>,
}

/// A DataForSEO Trends item, discriminated by its `type` field.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KeywordsDataApiDataforseoTrendsItem {
    /// Interest over time (`dataforseo_trends_graph`).
    DataforseoTrendsGraph(KeywordsDataApiDataforseoTrendsGraph),
    /// Interest broken down by region (`subregion_interests`).
    SubregionInterests(KeywordsDataApiDataforseoTrendsSubregionInterests),
    /// Interest broken down by age and gender (`demography`).
    Demography(KeywordsDataApiDataforseoTrendsDemography),
    /// Any item type not yet modeled by this crate.
    #[serde(other)]
    #[default]
    Unknown,
}

/// `dataforseo_trends_graph` — interest over time.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsGraph {
    /// Position of this element within the result.
    pub position: Option<i32>,
    /// Keywords the graph covers, matching the order of each data point's values.
    pub keywords: Option<Vec<String>>,
    /// Interest data points over time.
    pub data: Option<Vec<KeywordsDataApiDataforseoTrendsGraphData>>,
    /// Average interest per keyword over the whole period.
    pub averages: Option<Vec<i64>>,
}

/// A single data point of a `dataforseo_trends_graph`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsGraphData {
    /// Start date of the data point in "yyyy-mm-dd" format.
    pub date_from: Option<String>,
    /// End date of the data point in "yyyy-mm-dd" format.
    pub date_to: Option<String>,
    /// Unix timestamp of the data point.
    pub timestamp: Option<i64>,
    /// Relative interest (0-100) per keyword.
    pub values: Option<Vec<i64>>,
}

/// `subregion_interests` — interest broken down by region.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsSubregionInterests {
    /// Position of this element within the result.
    pub position: Option<i32>,
    /// Keywords the regional breakdown covers.
    pub keywords: Option<Vec<String>>,
    /// Per-keyword regional interest values.
    pub interests: Option<Vec<KeywordsDataApiDataforseoTrendsInterest>>,
    /// Cross-keyword comparison; shape varies, kept as raw JSON.
    pub interests_comparison: Option<Value>,
}

/// Per-keyword regional interest values.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsInterest {
    /// The keyword these regional values belong to.
    pub keyword: Option<String>,
    /// Interest value for each region.
    pub values: Option<Vec<KeywordsDataApiDataforseoTrendsInterestValue>>,
}

/// A single region's interest value.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsInterestValue {
    /// Geo identifier of the region.
    pub geo_id: Option<String>,
    /// Human-readable region name.
    pub geo_name: Option<String>,
    /// Relative interest (0-100) for the region.
    pub value: Option<i32>,
}

/// `demography` — interest broken down by age and gender.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsDemography {
    /// Position of this element within the result.
    pub position: Option<i32>,
    /// Keywords the demographic breakdown covers.
    pub keywords: Option<Vec<String>>,
    /// Age and gender breakdown per keyword.
    pub demography: Option<KeywordsDataApiDataforseoTrendsDemographyData>,
    /// Cross-keyword comparison; shape varies, kept as raw JSON.
    pub demography_comparison: Option<Value>,
}

/// Age and gender breakdown of a `demography` item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsDemographyData {
    /// Interest by age group per keyword.
    pub age: Option<Vec<KeywordsDataApiDataforseoTrendsDemographyKeyword>>,
    /// Interest by gender per keyword.
    pub gender: Option<Vec<KeywordsDataApiDataforseoTrendsDemographyKeyword>>,
}

/// Per-keyword demographic values.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsDemographyKeyword {
    /// The keyword these demographic values belong to.
    pub keyword: Option<String>,
    /// Interest value for each demographic segment.
    pub values: Option<Vec<KeywordsDataApiDataforseoTrendsDemographyValue>>,
}

/// A single demographic segment value.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsDemographyValue {
    /// Segment label, e.g. "18-24" for age or "male"/"female" for gender.
    #[serde(rename = "type")]
    pub segment_type: Option<String>,
    /// Relative interest (0-100) for the segment.
    pub value: Option<i32>,
}
