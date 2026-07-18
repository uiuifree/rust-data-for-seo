//! DataForSEO Labs API — keyword, domain and market analytics.
//! See <https://docs.dataforseo.com/v3/dataforseo_labs/overview/>.

mod amazon;
mod apple;
mod google;

pub use amazon::*;
pub use apple::*;
pub use google::*;

use crate::entity::{
    DataForSeoLabsCategory, DataForSeoLabsIdListItem, DataForSeoLabsLocation, DataForSeoLabsStatus,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

/// Entry point for the DataForSEO Labs API domain.
pub struct DataForSeoLabsApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Returns the DataForSEO Labs API.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/overview/>.
    pub fn dataforseo_labs(&self) -> DataForSeoLabsApi<'_> {
        DataForSeoLabsApi { client: self }
    }
}

impl<'a> DataForSeoLabsApi<'a> {
    /// Google Labs endpoints (keyword research, competitors, app data, ...).
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/>.
    pub fn google(&self) -> DataForSeoLabsApiGoogle<'a> {
        DataForSeoLabsApiGoogle {
            client: self.client,
        }
    }

    /// Amazon Labs endpoints (product keywords and competitors).
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/>.
    pub fn amazon(&self) -> DataForSeoLabsApiAmazon<'a> {
        DataForSeoLabsApiAmazon {
            client: self.client,
        }
    }

    /// Apple App Store Labs endpoints.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/apple/>.
    pub fn apple(&self) -> DataForSeoLabsApiApple<'a> {
        DataForSeoLabsApiApple {
            client: self.client,
        }
    }

    /// Lists ids of previously posted Labs tasks.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/id_list/>.
    pub async fn id_list(
        &self,
        data: Vec<DataForSeoLabsIdListRequest>,
    ) -> DataForSeoApiResponse<DataForSeoLabsIdListItem> {
        self.client
            .http_post("/v3/dataforseo_labs/id_list", &data)
            .await
    }

    /// Lists locations and the languages available for each.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/locations_and_languages/>.
    pub async fn locations_and_languages(&self) -> DataForSeoApiResponse<DataForSeoLabsLocation> {
        self.client
            .http_get("/v3/dataforseo_labs/locations_and_languages")
            .await
    }

    /// Lists the product categories used across Labs endpoints.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/categories/>.
    pub async fn categories(&self) -> DataForSeoApiResponse<DataForSeoLabsCategory> {
        self.client.http_get("/v3/dataforseo_labs/categories").await
    }

    /// Returns Labs API availability and rate-limit information.
    /// See <https://docs.dataforseo.com/v3/dataforseo_labs/status/>.
    pub async fn status(&self) -> DataForSeoApiResponse<DataForSeoLabsStatus> {
        self.client.http_get("/v3/dataforseo_labs/status").await
    }
}

/// Request body for the `id_list` endpoint.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsIdListRequest {
    /// Start of the time window (UTC).
    pub datetime_from: String,
    /// End of the time window (UTC).
    pub datetime_to: String,
    /// Maximum number of returned items (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set, for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Sort order of the returned ids: `"asc"` or `"desc"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Whether to include each task's original POST metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
}

impl DataForSeoLabsIdListRequest {
    /// Creates a request over the inclusive `[datetime_from, datetime_to]` window (UTC).
    pub fn new(datetime_from: &str, datetime_to: &str) -> DataForSeoLabsIdListRequest {
        DataForSeoLabsIdListRequest {
            datetime_from: datetime_from.to_string(),
            datetime_to: datetime_to.to_string(),
            ..Default::default()
        }
    }
}
