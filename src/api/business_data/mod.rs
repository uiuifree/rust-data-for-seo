//! Business Data API domain: reviews and profiles from Google, Trustpilot,
//! Tripadvisor, social media, and the business listings database.
//! See <https://docs.dataforseo.com/v3/business_data/overview/>.

use crate::entity::{
    BusinessDataApiErrorItem, BusinessDataApiIdList, BusinessDataApiIdListRequest,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};

mod business_listings;
mod google;
mod social_media;
mod tripadvisor;
mod trustpilot;

pub use business_listings::*;
pub use google::*;
pub use social_media::*;
pub use tripadvisor::*;
pub use trustpilot::*;

/// Entry point for the Business Data API domain.
/// See <https://docs.dataforseo.com/v3/business_data/overview/>.
pub struct BusinessDataApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Access the Business Data API (Google, Trustpilot, Tripadvisor, ...).
    /// See <https://docs.dataforseo.com/v3/business_data/overview/>.
    pub fn business_data(&self) -> BusinessDataApi<'_> {
        BusinessDataApi { client: self }
    }
}

impl BusinessDataApi<'_> {
    /// Lists the ids of tasks set within the requested time range.
    /// See <https://docs.dataforseo.com/v3/business_data/id_list/>.
    pub async fn id_list(
        &self,
        data: Vec<BusinessDataApiIdListRequest>,
    ) -> DataForSeoApiResponse<BusinessDataApiIdList> {
        self.client
            .http_post("/v3/business_data/id_list", &data)
            .await
    }

    /// Lists errors that occurred while processing tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/errors/>.
    pub async fn errors(
        &self,
        data: Vec<BusinessDataApiIdListRequest>,
    ) -> DataForSeoApiResponse<BusinessDataApiErrorItem> {
        self.client
            .http_post("/v3/business_data/errors", &data)
            .await
    }
}
