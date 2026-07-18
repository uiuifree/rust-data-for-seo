use crate::api::domain_analytics::DomainAnalyticsApi;
use crate::entity::DomainAnalyticsApiWhoisOverview;
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl DomainAnalyticsApi<'_> {
    /// Returns the Whois sub-API builder.
    pub fn whois(&self) -> DomainAnalyticsApiWhois<'_> {
        DomainAnalyticsApiWhois {
            client: self.client,
        }
    }
}

/// Builder for the [Domain Analytics Whois](https://docs.dataforseo.com/v3/domain_analytics/whois/overview/live/) endpoints.
pub struct DomainAnalyticsApiWhois<'a> {
    client: &'a DataForSeoClient,
}

impl DomainAnalyticsApiWhois<'_> {
    /// Returns Whois records enriched with ranking, traffic, and backlink stats.
    /// See <https://docs.dataforseo.com/v3/domain_analytics/whois/overview/live/>.
    pub async fn overview(
        &self,
        data: Vec<DomainAnalyticsApiWhoisOverviewPost>,
    ) -> DataForSeoApiResponse<DomainAnalyticsApiWhoisOverview> {
        self.client
            .http_post("/v3/domain_analytics/whois/overview/live", &data)
            .await
    }
}

/// Request body for the Whois `overview` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiWhoisOverviewPost {
    /// Maximum number of returned domains (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset in the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Pagination token for fetching large result sets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_token: Option<String>,
    /// Result filters, each a `[field, operator, value]` array (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules such as `["domain,asc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DomainAnalyticsApiWhoisOverviewPost {
    /// Creates an empty request (all parameters optional).
    pub fn new() -> DomainAnalyticsApiWhoisOverviewPost {
        DomainAnalyticsApiWhoisOverviewPost::default()
    }
}
