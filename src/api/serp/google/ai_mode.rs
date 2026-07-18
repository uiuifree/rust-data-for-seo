use crate::entity::SerpApiGoogleAiModeTaskAdvanced;
use crate::{DataForSeoApiResponse, SerpApiGoogle};
use serde::{Deserialize, Serialize};

/// Google AI Mode SERP endpoint. AI Mode is a live-only endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/ai_mode/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/ai_mode/live/advanced/>
    pub async fn ai_mode_live_advanced(
        &self,
        data: Vec<SerpApiGoogleAiModeLiveRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleAiModeTaskAdvanced> {
        self.client
            .http_post("/v3/serp/google/ai_mode/live/advanced", &data)
            .await
    }
}

/// Request body for Google AI Mode live/advanced.
/// See <https://docs.dataforseo.com/v3/serp/google/ai_mode/live/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleAiModeLiveRequest {
    /// Search term the result was returned for.
    pub keyword: String,
    /// Full location name (e.g. "London,England,United Kingdom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: i32,
    /// GPS coordinates the search was run for ("lat,lng,zoom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    pub language_code: String,
    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Calculate rectangles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculate_rectangles: Option<bool>,
    /// Browser screen width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_width: Option<i32>,
    /// Browser screen height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_height: Option<i32>,
    /// Browser screen resolution ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_resolution_ratio: Option<i32>,
    /// User-defined identifier echoed back on the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl SerpApiGoogleAiModeLiveRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleAiModeLiveRequest {
            language_code,
            location_code,
            ..SerpApiGoogleAiModeLiveRequest::default()
        }
    }
}
