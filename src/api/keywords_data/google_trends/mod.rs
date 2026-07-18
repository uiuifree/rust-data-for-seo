use crate::DataForSeoApiResponse;
use crate::DataForSeoClient;
use crate::api::keywords_data::{KeywordsDataApi, KeywordsDataApiLocation};
use crate::entity::{KeywordsDataApiGoogleTrendsCategory, KeywordsDataApiLanguage};

mod explore;

pub use explore::*;

impl KeywordsDataApi<'_> {
    /// Google Trends endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/google_trends/overview/>.
    pub fn google_trends(&self) -> KeywordsDataApiGoogleTrends<'_> {
        KeywordsDataApiGoogleTrends {
            client: self.client,
        }
    }
}

/// Google Trends endpoints.
pub struct KeywordsDataApiGoogleTrends<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl KeywordsDataApiGoogleTrends<'_> {
    /// Locations supported by the Google Trends endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/google_trends/locations/>.
    pub async fn locations(&self) -> DataForSeoApiResponse<KeywordsDataApiLocation> {
        self.client
            .http_get("/v3/keywords_data/google_trends/locations")
            .await
    }
    /// Languages supported by the Google Trends endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/google_trends/languages/>.
    pub async fn languages(&self) -> DataForSeoApiResponse<KeywordsDataApiLanguage> {
        self.client
            .http_get("/v3/keywords_data/google_trends/languages")
            .await
    }
    /// Categories supported by the Google Trends endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/google_trends/categories/>.
    pub async fn categories(&self) -> DataForSeoApiResponse<KeywordsDataApiGoogleTrendsCategory> {
        self.client
            .http_get("/v3/keywords_data/google_trends/categories")
            .await
    }
}
