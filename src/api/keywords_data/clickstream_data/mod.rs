use crate::DataForSeoApiResponse;
use crate::DataForSeoClient;
use crate::api::keywords_data::KeywordsDataApi;
use crate::entity::{
    KeywordsDataApiClickstreamBulkSearchVolume, KeywordsDataApiClickstreamGlobalSearchVolume,
    KeywordsDataApiClickstreamLocation, KeywordsDataApiClickstreamSearchVolume,
};
use serde::{Deserialize, Serialize};

impl KeywordsDataApi<'_> {
    /// Clickstream Data endpoints (clickstream-based search volume).
    /// See <https://docs.dataforseo.com/v3/keywords_data/clickstream_data/overview/>.
    pub fn clickstream_data(&self) -> KeywordsDataApiClickstream<'_> {
        KeywordsDataApiClickstream {
            client: self.client,
        }
    }
}

/// Clickstream Data endpoints.
pub struct KeywordsDataApiClickstream<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl KeywordsDataApiClickstream<'_> {
    /// Locations and their available languages for Clickstream Data.
    /// See <https://docs.dataforseo.com/v3/keywords_data/clickstream_data/locations_and_languages/>.
    pub async fn locations_and_languages(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiClickstreamLocation> {
        self.client
            .http_get("/v3/keywords_data/clickstream_data/locations_and_languages")
            .await
    }
    /// Retrieves clickstream-based search volume synchronously.
    /// See <https://docs.dataforseo.com/v3/keywords_data/clickstream_data/dataforseo_search_volume/live/>.
    pub async fn dataforseo_search_volume_live(
        &self,
        data: Vec<KeywordsDataApiClickstreamSearchVolumeRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiClickstreamSearchVolume> {
        self.client
            .http_post(
                "/v3/keywords_data/clickstream_data/dataforseo_search_volume/live",
                &data,
            )
            .await
    }
    /// Retrieves worldwide clickstream-based search volume synchronously.
    /// See <https://docs.dataforseo.com/v3/keywords_data/clickstream_data/global_search_volume/live/>.
    pub async fn global_search_volume_live(
        &self,
        data: Vec<KeywordsDataApiClickstreamGlobalSearchVolumeRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiClickstreamGlobalSearchVolume> {
        self.client
            .http_post(
                "/v3/keywords_data/clickstream_data/global_search_volume/live",
                &data,
            )
            .await
    }
    /// Retrieves clickstream-based search volume for up to 1000 keywords synchronously.
    /// See <https://docs.dataforseo.com/v3/keywords_data/clickstream_data/bulk_search_volume/live/>.
    pub async fn bulk_search_volume_live(
        &self,
        data: Vec<KeywordsDataApiClickstreamBulkSearchVolumeRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiClickstreamBulkSearchVolume> {
        self.client
            .http_post(
                "/v3/keywords_data/clickstream_data/bulk_search_volume/live",
                &data,
            )
            .await
    }
}

/// Request body for the Clickstream `dataforseo_search_volume` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiClickstreamSearchVolumeRequest {
    /// Keywords to look up (up to 1000 per request).
    pub keywords: Vec<String>,
    /// Full location name; alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Numeric location identifier; alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full language name; alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code, e.g. "en"; alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Blend clickstream data into the returned search volume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_clickstream: Option<bool>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiClickstreamSearchVolumeRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> Self {
        KeywordsDataApiClickstreamSearchVolumeRequest {
            keywords,
            ..KeywordsDataApiClickstreamSearchVolumeRequest::default()
        }
    }
}

/// Request body for the Clickstream `global_search_volume` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiClickstreamGlobalSearchVolumeRequest {
    /// Keywords to look up (up to 1000; minimum 3 characters each).
    pub keywords: Vec<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiClickstreamGlobalSearchVolumeRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> Self {
        KeywordsDataApiClickstreamGlobalSearchVolumeRequest {
            keywords,
            ..KeywordsDataApiClickstreamGlobalSearchVolumeRequest::default()
        }
    }
}

/// Request body for the Clickstream `bulk_search_volume` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiClickstreamBulkSearchVolumeRequest {
    /// Keywords to look up (up to 1000; minimum 3 characters each).
    pub keywords: Vec<String>,
    /// Full location name; alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Numeric location identifier; alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiClickstreamBulkSearchVolumeRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> Self {
        KeywordsDataApiClickstreamBulkSearchVolumeRequest {
            keywords,
            ..KeywordsDataApiClickstreamBulkSearchVolumeRequest::default()
        }
    }
}
