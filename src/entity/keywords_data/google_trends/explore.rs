use serde::{Deserialize, Serialize};

/// A single Google Trends Explore result (one per task).
/// See <https://docs.dataforseo.com/v3/keywords_data/google_trends/explore/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsExplore {
    /// Keywords the trends data was requested for.
    pub keywords: Option<Vec<String>>,
    /// Trends data source, e.g. "web", "news", "youtube", "images" or "froogle".
    #[serde(rename = "type")]
    pub explore_type: Option<String>,
    /// Location code the data was requested for.
    pub location_code: Option<i32>,
    /// Language code the data was requested for.
    pub language_code: Option<String>,
    /// URL of the Google Trends page reflecting these parameters.
    pub check_url: Option<String>,
    /// UTC timestamp when the data was collected.
    pub datetime: Option<String>,
    /// Number of elements in `items`.
    pub items_count: Option<i32>,
    /// Trends elements returned for the request.
    pub items: Option<Vec<KeywordsDataApiGoogleTrendsItem>>,
}

/// A Google Trends Explore item, discriminated by its `type` field.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KeywordsDataApiGoogleTrendsItem {
    /// Interest over time (`google_trends_graph`).
    GoogleTrendsGraph(KeywordsDataApiGoogleTrendsGraph),
    /// Interest by region (`google_trends_map`).
    GoogleTrendsMap(KeywordsDataApiGoogleTrendsMap),
    /// Related topics (`google_trends_topics_list`).
    GoogleTrendsTopicsList(KeywordsDataApiGoogleTrendsTopicsList),
    /// Related queries (`google_trends_queries_list`).
    GoogleTrendsQueriesList(KeywordsDataApiGoogleTrendsQueriesList),
    /// Any item type not yet modeled by this crate.
    #[serde(other)]
    #[default]
    Unknown,
}

/// `google_trends_graph` — interest over time.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsGraph {
    /// Position of this element within the result.
    pub position: Option<i32>,
    /// Human-readable title of the element.
    pub title: Option<String>,
    /// Keywords the graph covers, matching the order of each data point's values.
    pub keywords: Option<Vec<String>>,
    /// Interest data points over time.
    pub data: Option<Vec<KeywordsDataApiGoogleTrendsGraphData>>,
    /// Average interest per keyword over the whole period.
    pub averages: Option<Vec<i64>>,
}

/// A single data point of a `google_trends_graph`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsGraphData {
    /// Start date of the data point in "yyyy-mm-dd" format.
    pub date_from: Option<String>,
    /// End date of the data point in "yyyy-mm-dd" format.
    pub date_to: Option<String>,
    /// Unix timestamp of the data point.
    pub timestamp: Option<i64>,
    /// Whether interest data is missing for this point.
    pub missing_data: Option<bool>,
    /// Relative interest (0-100) per keyword.
    pub values: Option<Vec<i64>>,
}

/// `google_trends_map` — interest by region.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsMap {
    /// Position of this element within the result.
    pub position: Option<i32>,
    /// Human-readable title of the element.
    pub title: Option<String>,
    /// Keywords the map covers, matching the order of each region's values.
    pub keywords: Option<Vec<String>>,
    /// Interest values per region.
    pub data: Option<Vec<KeywordsDataApiGoogleTrendsMapData>>,
}

/// A single region of a `google_trends_map`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsMapData {
    /// Google geo identifier of the region.
    pub geo_id: Option<String>,
    /// Human-readable region name.
    pub geo_name: Option<String>,
    /// Relative interest (0-100) per keyword.
    pub values: Option<Vec<i64>>,
    /// Index into `values` of the keyword with the highest interest.
    pub max_value_index: Option<i32>,
}

/// `google_trends_topics_list` — top and rising related topics.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsTopicsList {
    /// Position of this element within the result.
    pub position: Option<i32>,
    /// Human-readable title of the element.
    pub title: Option<String>,
    /// Keywords the related topics were derived from.
    pub keywords: Option<Vec<String>>,
    /// Top and rising related topics.
    pub data: Option<KeywordsDataApiGoogleTrendsTopicsData>,
}

/// Top and rising topics of a `google_trends_topics_list`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsTopicsData {
    /// Consistently most popular related topics.
    pub top: Option<Vec<KeywordsDataApiGoogleTrendsTopic>>,
    /// Topics with the fastest-growing interest.
    pub rising: Option<Vec<KeywordsDataApiGoogleTrendsTopic>>,
}

/// A related topic entry.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsTopic {
    /// Google identifier of the topic.
    pub topic_id: Option<String>,
    /// Human-readable topic title.
    pub topic_title: Option<String>,
    /// Category of the topic.
    pub topic_type: Option<String>,
    /// Relative interest, or "Breakout" for rising topics.
    pub value: Option<String>,
}

/// `google_trends_queries_list` — top and rising related queries.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsQueriesList {
    /// Position of this element within the result.
    pub position: Option<i32>,
    /// Human-readable title of the element.
    pub title: Option<String>,
    /// Keywords the related queries were derived from.
    pub keywords: Option<Vec<String>>,
    /// Top and rising related queries.
    pub data: Option<KeywordsDataApiGoogleTrendsQueriesData>,
}

/// Top and rising queries of a `google_trends_queries_list`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsQueriesData {
    /// Consistently most popular related queries.
    pub top: Option<Vec<KeywordsDataApiGoogleTrendsQuery>>,
    /// Queries with the fastest-growing interest.
    pub rising: Option<Vec<KeywordsDataApiGoogleTrendsQuery>>,
}

/// A related query entry.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsQuery {
    /// The related query text.
    pub query: Option<String>,
    /// Relative interest, or "Breakout" for rising queries.
    pub value: Option<String>,
}
