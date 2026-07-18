//! AI Keyword Data sub-API.
//!
//! See <https://docs.dataforseo.com/v3/ai_optimization/ai_keyword_data/overview/>.

use crate::entity::{AiOptimizationKeywordDataSearchVolume, AiOptimizationLocationLanguage};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

/// Endpoints under `/v3/ai_optimization/ai_keyword_data/`.
pub struct AiOptimizationApiKeywordData<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl AiOptimizationApiKeywordData<'_> {
    /// Lists the locations and languages supported by AI Keyword Data.
    ///
    /// `GET /v3/ai_optimization/ai_keyword_data/locations_and_languages`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/ai_keyword_data/locations_and_languages/>.
    pub async fn locations_and_languages(
        &self,
    ) -> DataForSeoApiResponse<AiOptimizationLocationLanguage> {
        self.client
            .http_get("/v3/ai_optimization/ai_keyword_data/locations_and_languages")
            .await
    }

    /// Returns AI search volume for the given keywords.
    ///
    /// `POST /v3/ai_optimization/ai_keyword_data/keywords_search_volume/live`
    /// See <https://docs.dataforseo.com/v3/ai_optimization/ai_keyword_data/keywords_search_volume/live/>.
    pub async fn keywords_search_volume(
        &self,
        data: Vec<AiOptimizationKeywordDataSearchVolumePost>,
    ) -> DataForSeoApiResponse<AiOptimizationKeywordDataSearchVolume> {
        self.client
            .http_post(
                "/v3/ai_optimization/ai_keyword_data/keywords_search_volume/live",
                &data,
            )
            .await
    }
}

/// Request body for `keywords_search_volume/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationKeywordDataSearchVolumePost {
    /// Keywords to look up (up to 1000, max 250 characters each).
    pub keywords: Vec<String>,
    /// Full name of the search location (alternative to `location_code`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Search location code (alternative to `location_name`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full name of the search language (alternative to `language_code`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Search language code (alternative to `language_name`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl AiOptimizationKeywordDataSearchVolumePost {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> AiOptimizationKeywordDataSearchVolumePost {
        AiOptimizationKeywordDataSearchVolumePost {
            keywords,
            ..Default::default()
        }
    }
}
