use crate::entity::{DomainAnalyticsApiErrorItem, DomainAnalyticsApiIdList};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

mod technologies;
mod whois;

pub use technologies::*;
pub use whois::*;

/// Entry point for the [Domain Analytics API](https://docs.dataforseo.com/v3/domain_analytics/).
pub struct DomainAnalyticsApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Returns the Domain Analytics API builder.
    pub fn domain_analytics(&self) -> DomainAnalyticsApi<'_> {
        DomainAnalyticsApi { client: self }
    }
}

impl DomainAnalyticsApi<'_> {
    /// Returns the list of task IDs completed within the requested time range.
    /// See <https://docs.dataforseo.com/v3/domain_analytics/id_list/>.
    pub async fn id_list(
        &self,
        data: Vec<DomainAnalyticsApiIdListPost>,
    ) -> DataForSeoApiResponse<DomainAnalyticsApiIdList> {
        self.client
            .http_post("/v3/domain_analytics/id_list", &data)
            .await
    }

    /// Returns tasks that responded with an error within the last 7 days.
    /// See <https://docs.dataforseo.com/v3/domain_analytics/errors/>.
    pub async fn errors(
        &self,
        data: Vec<DomainAnalyticsApiErrorsPost>,
    ) -> DataForSeoApiResponse<DomainAnalyticsApiErrorItem> {
        self.client
            .http_post("/v3/domain_analytics/errors", &data)
            .await
    }
}

/// Request body for the `id_list` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiIdListPost {
    /// Start of the range in UTC (`yyyy-mm-dd hh-mm-ss +00:00`).
    pub datetime_from: String,
    /// End of the range in UTC (`yyyy-mm-dd hh-mm-ss +00:00`).
    pub datetime_to: String,
    /// Maximum number of task IDs to return (default 1000, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset in the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Sort direction by execution time: `asc` (default) or `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Whether to include original request metadata in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
}

impl DomainAnalyticsApiIdListPost {
    /// Creates a request for the given UTC datetime range (`yyyy-mm-dd hh-mm-ss +00:00`).
    pub fn new(datetime_from: String, datetime_to: String) -> DomainAnalyticsApiIdListPost {
        DomainAnalyticsApiIdListPost {
            datetime_from,
            datetime_to,
            ..Default::default()
        }
    }
}

/// Request body for the `errors` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiErrorsPost {
    /// Maximum number of errored tasks to return (default 1000, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset in the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Restrict results to a specific API function (e.g. `domain_analytics/task_get`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_function: Option<String>,
    /// Start of the range in UTC (`yyyy-mm-dd hh-mm-ss +00:00`), last 7 days only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_from: Option<String>,
    /// End of the range in UTC (`yyyy-mm-dd hh-mm-ss +00:00`), last 7 days only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_to: Option<String>,
}

impl DomainAnalyticsApiErrorsPost {
    /// Creates an empty request (all filters optional).
    pub fn new() -> DomainAnalyticsApiErrorsPost {
        DomainAnalyticsApiErrorsPost::default()
    }
}
