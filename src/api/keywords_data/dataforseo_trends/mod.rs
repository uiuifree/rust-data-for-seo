use crate::DataForSeoApiResponse;
use crate::DataForSeoClient;
use crate::api::keywords_data::{KeywordsDataApi, KeywordsDataApiLocation};
use crate::entity::KeywordsDataApiDataforseoTrendsResult;
use serde::{Deserialize, Serialize};

impl KeywordsDataApi<'_> {
    /// DataForSEO Trends endpoints (proprietary clickstream-based trends).
    /// See <https://docs.dataforseo.com/v3/keywords_data/dataforseo_trends/overview/>.
    pub fn dataforseo_trends(&self) -> KeywordsDataApiDataforseoTrends<'_> {
        KeywordsDataApiDataforseoTrends {
            client: self.client,
        }
    }
}

/// DataForSEO Trends endpoints.
pub struct KeywordsDataApiDataforseoTrends<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl KeywordsDataApiDataforseoTrends<'_> {
    /// Locations supported by the DataForSEO Trends endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/dataforseo_trends/locations/>.
    pub async fn locations(&self) -> DataForSeoApiResponse<KeywordsDataApiLocation> {
        self.client
            .http_get("/v3/keywords_data/dataforseo_trends/locations")
            .await
    }
    /// Retrieves keyword popularity over time synchronously.
    /// See <https://docs.dataforseo.com/v3/keywords_data/dataforseo_trends/explore/live/>.
    pub async fn explore_live(
        &self,
        data: Vec<KeywordsDataApiDataforseoTrendsRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiDataforseoTrendsResult> {
        self.client
            .http_post("/v3/keywords_data/dataforseo_trends/explore/live", &data)
            .await
    }
    /// Retrieves keyword popularity by subregion synchronously.
    /// See <https://docs.dataforseo.com/v3/keywords_data/dataforseo_trends/subregion_interests/live/>.
    pub async fn subregion_interests_live(
        &self,
        data: Vec<KeywordsDataApiDataforseoTrendsRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiDataforseoTrendsResult> {
        self.client
            .http_post(
                "/v3/keywords_data/dataforseo_trends/subregion_interests/live",
                &data,
            )
            .await
    }
    /// Retrieves keyword popularity by demographics synchronously.
    /// See <https://docs.dataforseo.com/v3/keywords_data/dataforseo_trends/demography/live/>.
    pub async fn demography_live(
        &self,
        data: Vec<KeywordsDataApiDataforseoTrendsRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiDataforseoTrendsResult> {
        self.client
            .http_post("/v3/keywords_data/dataforseo_trends/demography/live", &data)
            .await
    }
    /// Retrieves graph, subregion and demographic data in one call.
    /// See <https://docs.dataforseo.com/v3/keywords_data/dataforseo_trends/merged_data/live/>.
    pub async fn merged_data_live(
        &self,
        data: Vec<KeywordsDataApiDataforseoTrendsRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiDataforseoTrendsResult> {
        self.client
            .http_post(
                "/v3/keywords_data/dataforseo_trends/merged_data/live",
                &data,
            )
            .await
    }
}

/// Request body shared by all DataForSEO Trends live endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiDataforseoTrendsRequest {
    /// Keywords to analyze (up to 5).
    pub keywords: Vec<String>,
    /// Full location name; alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Numeric location identifier; alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Trends data source: `web`, `news` or `ecommerce`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub trends_type: Option<String>,
    /// Start of the range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End of the range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Preset range, e.g. "past_30_days"; alternative to `date_from`/`date_to`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
    /// Data blocks to return; used by the `merged_data` endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiDataforseoTrendsRequest {
    /// Creates a request for the given keywords (max 5).
    pub fn new(keywords: Vec<String>) -> Self {
        KeywordsDataApiDataforseoTrendsRequest {
            keywords,
            ..KeywordsDataApiDataforseoTrendsRequest::default()
        }
    }
}
