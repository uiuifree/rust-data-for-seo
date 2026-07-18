//! Business Listings endpoints of the Business Data API.
//! See <https://docs.dataforseo.com/v3/business_data/business_listings/overview/>.

use crate::api::business_data::BusinessDataApi;
use crate::entity::{
    BusinessDataApiBusinessListingsCategory, BusinessDataApiBusinessListingsLocation,
    BusinessDataApiBusinessListingsSearchRequest, BusinessDataApiBusinessListingsSearchResult,
    BusinessDataApiCategoriesAggregationRequest, BusinessDataApiCategoriesAggregationResult,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};

impl BusinessDataApi<'_> {
    /// Returns the Business Listings sub-API of the Business Data domain.
    /// See <https://docs.dataforseo.com/v3/business_data/business_listings/overview/>.
    pub fn business_listings(&self) -> BusinessDataApiBusinessListings<'_> {
        BusinessDataApiBusinessListings {
            client: self.client,
        }
    }
}

/// Business Listings endpoints: search, aggregation, and reference data.
pub struct BusinessDataApiBusinessListings<'a> {
    client: &'a DataForSeoClient,
}

impl BusinessDataApiBusinessListings<'_> {
    /// Lists locations available for Business Listings search.
    /// See <https://docs.dataforseo.com/v3/business_data/business_listings/locations/>.
    pub async fn locations(
        &self,
    ) -> DataForSeoApiResponse<BusinessDataApiBusinessListingsLocation> {
        self.client
            .http_get("/v3/business_data/business_listings/locations")
            .await
    }

    /// Lists business categories available for filtering.
    /// See <https://docs.dataforseo.com/v3/business_data/business_listings/categories/>.
    pub async fn categories(
        &self,
    ) -> DataForSeoApiResponse<BusinessDataApiBusinessListingsCategory> {
        self.client
            .http_get("/v3/business_data/business_listings/categories")
            .await
    }

    /// Searches business listings synchronously.
    /// See <https://docs.dataforseo.com/v3/business_data/business_listings/search/live/>.
    pub async fn search_live(
        &self,
        data: Vec<BusinessDataApiBusinessListingsSearchRequest>,
    ) -> DataForSeoApiResponse<BusinessDataApiBusinessListingsSearchResult> {
        self.client
            .http_post("/v3/business_data/business_listings/search/live", &data)
            .await
    }

    /// Returns aggregated category statistics synchronously.
    /// See <https://docs.dataforseo.com/v3/business_data/business_listings/categories_aggregation/live/>.
    pub async fn categories_aggregation_live(
        &self,
        data: Vec<BusinessDataApiCategoriesAggregationRequest>,
    ) -> DataForSeoApiResponse<BusinessDataApiCategoriesAggregationResult> {
        self.client
            .http_post(
                "/v3/business_data/business_listings/categories_aggregation/live",
                &data,
            )
            .await
    }
}
