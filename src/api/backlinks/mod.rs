use crate::entity::{
    BacklinksApiAnchor, BacklinksApiBacklinks, BacklinksApiBulkBacklinks,
    BacklinksApiBulkNewLostBacklinks, BacklinksApiBulkNewLostReferringDomains,
    BacklinksApiBulkPagesSummary, BacklinksApiBulkRanks, BacklinksApiBulkReferringDomains,
    BacklinksApiBulkSpamScore, BacklinksApiCompetitors, BacklinksApiDomainIntersection,
    BacklinksApiDomainPages, BacklinksApiDomainPagesSummary, BacklinksApiError,
    BacklinksApiHistory, BacklinksApiIdListItem, BacklinksApiIndex, BacklinksApiPageIntersection,
    BacklinksApiReferringDomains, BacklinksApiReferringNetworks, BacklinksApiSummary,
    BacklinksApiTimeseriesNewLostSummary, BacklinksApiTimeseriesSummary,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Client for the DataForSEO Backlinks API.
/// See <https://docs.dataforseo.com/v3/backlinks/overview/>.
pub struct BacklinksApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Returns the Backlinks API client.
    pub fn backlinks(&self) -> BacklinksApi<'_> {
        BacklinksApi { client: self }
    }
}

impl BacklinksApi<'_> {
    /// Total volume of the DataForSEO backlinks database.
    /// See <https://docs.dataforseo.com/v3/backlinks/index/>.
    pub async fn index(&self) -> DataForSeoApiResponse<BacklinksApiIndex> {
        self.client.http_get("/v3/backlinks/index").await
    }

    /// Filterable fields available across Backlinks endpoints.
    /// See <https://docs.dataforseo.com/v3/backlinks/filters/>.
    pub async fn available_filters(&self) -> DataForSeoApiResponse<Value> {
        self.client
            .http_get("/v3/backlinks/available_filters")
            .await
    }

    /// Aggregated backlink metrics for a target.
    /// See <https://docs.dataforseo.com/v3/backlinks/summary/live/>.
    pub async fn summary(
        &self,
        data: Vec<BacklinksApiSummaryPost>,
    ) -> DataForSeoApiResponse<BacklinksApiSummary> {
        self.client
            .http_post("/v3/backlinks/summary/live", &data)
            .await
    }

    /// Historical backlink metrics for a target.
    /// See <https://docs.dataforseo.com/v3/backlinks/history/live/>.
    pub async fn history(
        &self,
        data: Vec<BacklinksApiHistoryPost>,
    ) -> DataForSeoApiResponse<BacklinksApiHistory> {
        self.client
            .http_post("/v3/backlinks/history/live", &data)
            .await
    }

    /// List of backlinks pointing to a target.
    /// See <https://docs.dataforseo.com/v3/backlinks/backlinks/live/>.
    pub async fn backlinks(
        &self,
        data: Vec<BacklinksApiBacklinksPost>,
    ) -> DataForSeoApiResponse<BacklinksApiBacklinks> {
        self.client
            .http_post("/v3/backlinks/backlinks/live", &data)
            .await
    }

    /// Anchor texts referring to a target.
    /// See <https://docs.dataforseo.com/v3/backlinks/anchors/live/>.
    pub async fn anchor(
        &self,
        data: Vec<BacklinksApiAnchorPost>,
    ) -> DataForSeoApiResponse<BacklinksApiAnchor> {
        self.client
            .http_post("/v3/backlinks/anchors/live", &data)
            .await
    }

    /// Pages of a domain that have backlinks.
    /// See <https://docs.dataforseo.com/v3/backlinks/domain_pages/live/>.
    pub async fn domain_pages(
        &self,
        data: Vec<BacklinksApiDomainPagesPost>,
    ) -> DataForSeoApiResponse<BacklinksApiDomainPages> {
        self.client
            .http_post("/v3/backlinks/domain_pages/live", &data)
            .await
    }

    /// Backlink summary per page of a domain.
    /// See <https://docs.dataforseo.com/v3/backlinks/domain_pages_summary/live/>.
    pub async fn domain_pages_summary(
        &self,
        data: Vec<BacklinksApiDomainPagesSummaryPost>,
    ) -> DataForSeoApiResponse<BacklinksApiDomainPagesSummary> {
        self.client
            .http_post("/v3/backlinks/domain_pages_summary/live", &data)
            .await
    }

    /// Domains linking to a target.
    /// See <https://docs.dataforseo.com/v3/backlinks/referring_domains/live/>.
    pub async fn referring_domains(
        &self,
        data: Vec<BacklinksApiReferringDomainsPost>,
    ) -> DataForSeoApiResponse<BacklinksApiReferringDomains> {
        self.client
            .http_post("/v3/backlinks/referring_domains/live", &data)
            .await
    }

    /// IP addresses or subnets linking to a target.
    /// See <https://docs.dataforseo.com/v3/backlinks/referring_networks/live/>.
    pub async fn referring_networks(
        &self,
        data: Vec<BacklinksApiReferringNetworksPost>,
    ) -> DataForSeoApiResponse<BacklinksApiReferringNetworks> {
        self.client
            .http_post("/v3/backlinks/referring_networks/live", &data)
            .await
    }

    /// Domains sharing backlinks with a target.
    /// See <https://docs.dataforseo.com/v3/backlinks/competitors/live/>.
    pub async fn competitors(
        &self,
        data: Vec<BacklinksApiCompetitorsPost>,
    ) -> DataForSeoApiResponse<BacklinksApiCompetitors> {
        self.client
            .http_post("/v3/backlinks/competitors/live", &data)
            .await
    }

    /// Domains linking to several targets at once.
    /// See <https://docs.dataforseo.com/v3/backlinks/domain_intersection/live/>.
    pub async fn domain_intersection(
        &self,
        data: Vec<BacklinksApiDomainIntersectionPost>,
    ) -> DataForSeoApiResponse<BacklinksApiDomainIntersection> {
        self.client
            .http_post("/v3/backlinks/domain_intersection/live", &data)
            .await
    }

    /// Pages linking to several targets at once.
    /// See <https://docs.dataforseo.com/v3/backlinks/page_intersection/live/>.
    pub async fn page_intersection(
        &self,
        data: Vec<BacklinksApiPageIntersectionPost>,
    ) -> DataForSeoApiResponse<BacklinksApiPageIntersection> {
        self.client
            .http_post("/v3/backlinks/page_intersection/live", &data)
            .await
    }

    /// Backlink metrics grouped over time.
    /// See <https://docs.dataforseo.com/v3/backlinks/timeseries_summary/live/>.
    pub async fn timeseries_summary(
        &self,
        data: Vec<BacklinksApiTimeseriesSummaryPost>,
    ) -> DataForSeoApiResponse<BacklinksApiTimeseriesSummary> {
        self.client
            .http_post("/v3/backlinks/timeseries_summary/live", &data)
            .await
    }

    /// New and lost backlink/referring-domain counts grouped over time.
    /// See <https://docs.dataforseo.com/v3/backlinks/timeseries_new_lost_summary/live/>.
    pub async fn timeseries_new_lost_summary(
        &self,
        data: Vec<BacklinksApiTimeseriesNewLostSummaryPost>,
    ) -> DataForSeoApiResponse<BacklinksApiTimeseriesNewLostSummary> {
        self.client
            .http_post("/v3/backlinks/timeseries_new_lost_summary/live", &data)
            .await
    }

    /// Rank for up to 1000 targets.
    /// See <https://docs.dataforseo.com/v3/backlinks/bulk_ranks/live/>.
    pub async fn bulk_ranks(
        &self,
        data: Vec<BacklinksApiBulkRanksPost>,
    ) -> DataForSeoApiResponse<BacklinksApiBulkRanks> {
        self.client
            .http_post("/v3/backlinks/bulk_ranks/live", &data)
            .await
    }

    /// Backlink counts for up to 1000 targets.
    /// See <https://docs.dataforseo.com/v3/backlinks/bulk_backlinks/live/>.
    pub async fn bulk_backlinks(
        &self,
        data: Vec<BacklinksApiBulkBacklinksPost>,
    ) -> DataForSeoApiResponse<BacklinksApiBulkBacklinks> {
        self.client
            .http_post("/v3/backlinks/bulk_backlinks/live", &data)
            .await
    }

    /// Spam score for up to 1000 targets.
    /// See <https://docs.dataforseo.com/v3/backlinks/bulk_spam_score/live/>.
    pub async fn bulk_spam_score(
        &self,
        data: Vec<BacklinksApiBulkSpamScorePost>,
    ) -> DataForSeoApiResponse<BacklinksApiBulkSpamScore> {
        self.client
            .http_post("/v3/backlinks/bulk_spam_score/live", &data)
            .await
    }

    /// New and lost backlink counts for up to 1000 targets.
    /// See <https://docs.dataforseo.com/v3/backlinks/bulk_new_lost_backlinks/live/>.
    pub async fn bulk_new_lost_backlinks(
        &self,
        data: Vec<BacklinksApiBulkNewLostBacklinksPost>,
    ) -> DataForSeoApiResponse<BacklinksApiBulkNewLostBacklinks> {
        self.client
            .http_post("/v3/backlinks/bulk_new_lost_backlinks/live", &data)
            .await
    }

    /// New and lost referring-domain counts for up to 1000 targets.
    /// See <https://docs.dataforseo.com/v3/backlinks/bulk_new_lost_referring_domains/live/>.
    pub async fn bulk_new_lost_referring_domains(
        &self,
        data: Vec<BacklinksApiBulkNewLostReferringDomainsPost>,
    ) -> DataForSeoApiResponse<BacklinksApiBulkNewLostReferringDomains> {
        self.client
            .http_post("/v3/backlinks/bulk_new_lost_referring_domains/live", &data)
            .await
    }

    /// Referring-domain counts for up to 1000 targets.
    /// See <https://docs.dataforseo.com/v3/backlinks/bulk_referring_domains/live/>.
    pub async fn bulk_referring_domains(
        &self,
        data: Vec<BacklinksApiBulkReferringDomainsPost>,
    ) -> DataForSeoApiResponse<BacklinksApiBulkReferringDomains> {
        self.client
            .http_post("/v3/backlinks/bulk_referring_domains/live", &data)
            .await
    }

    /// Backlink summary for up to 1000 pages.
    /// See <https://docs.dataforseo.com/v3/backlinks/bulk_pages_summary/live/>.
    pub async fn bulk_pages_summary(
        &self,
        data: Vec<BacklinksApiBulkPagesSummaryPost>,
    ) -> DataForSeoApiResponse<BacklinksApiBulkPagesSummary> {
        self.client
            .http_post("/v3/backlinks/bulk_pages_summary/live", &data)
            .await
    }

    /// Errors of Backlinks tasks made in the past 7 days.
    /// See <https://docs.dataforseo.com/v3/backlinks/errors/>.
    pub async fn errors(
        &self,
        data: Vec<BacklinksApiErrorsPost>,
    ) -> DataForSeoApiResponse<BacklinksApiError> {
        self.client.http_post("/v3/backlinks/errors", &data).await
    }

    /// List of Backlinks task ids within a datetime range.
    /// See <https://docs.dataforseo.com/v3/backlinks/id_list/>.
    pub async fn id_list(
        &self,
        data: Vec<BacklinksApiIdListPost>,
    ) -> DataForSeoApiResponse<BacklinksApiIdListItem> {
        self.client.http_post("/v3/backlinks/id_list", &data).await
    }
}

/// Request for the Summary endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiSummaryPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Whether to include indirect (redirect/canonical) links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_indirect_links: Option<bool>,
    /// Whether to exclude backlinks from the target's own subdomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_internal_backlinks: Option<bool>,
    /// Maximum number of elements within internal arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// Which backlinks to include: `all`, `live`, or `lost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_status_type: Option<String>,
    /// Filter applied to the underlying backlinks by any backlink field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_filters: Option<Vec<Value>>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiSummaryPost {
    /// Builds a request for the given target (domain, subdomain, or webpage).
    pub fn new(target: &str) -> BacklinksApiSummaryPost {
        BacklinksApiSummaryPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the History endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiHistoryPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Start date of the range (`yyyy-mm-dd`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End date of the range (`yyyy-mm-dd`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiHistoryPost {
    /// Builds a request for the given target.
    pub fn new(target: &str) -> BacklinksApiHistoryPost {
        BacklinksApiHistoryPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the Backlinks endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBacklinksPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Grouping mode: `as_is`, `one_per_domain`, or `one_per_anchor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Custom grouping definition with `field` and `value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mode: Option<Value>,
    /// Result filtering conditions (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules, e.g. `["rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Pagination token for retrieving results beyond the offset limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_after_token: Option<String>,
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Which backlinks to include: `all`, `live`, or `lost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_status_type: Option<String>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Whether to include indirect (redirect/canonical) links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_indirect_links: Option<bool>,
    /// Whether to exclude backlinks from the target's own subdomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_internal_backlinks: Option<bool>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiBacklinksPost {
    /// Builds a request for the given target.
    pub fn new(target: &str) -> BacklinksApiBacklinksPost {
        BacklinksApiBacklinksPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the Anchors endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiAnchorPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of elements within internal arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// Which backlinks to include: `all`, `live`, or `lost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_status_type: Option<String>,
    /// Result filtering conditions (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules, e.g. `["rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Filter applied to the underlying backlinks by any backlink field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_filters: Option<Vec<Value>>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Whether to include indirect (redirect/canonical) links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_indirect_links: Option<bool>,
    /// Whether to exclude backlinks from the target's own subdomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_internal_backlinks: Option<bool>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiAnchorPost {
    /// Builds a request for the given target.
    pub fn new(target: &str) -> BacklinksApiAnchorPost {
        BacklinksApiAnchorPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the Domain Pages endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiDomainPagesPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of elements within internal arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// Which backlinks to include: `all`, `live`, or `lost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_status_type: Option<String>,
    /// Result filtering conditions (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules, e.g. `["rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Filter applied to the underlying backlinks by any backlink field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_filters: Option<Vec<Value>>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Whether to exclude backlinks from the target's own subdomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_internal_backlinks: Option<bool>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiDomainPagesPost {
    /// Builds a request for the given target (domain or subdomain).
    pub fn new(target: &str) -> BacklinksApiDomainPagesPost {
        BacklinksApiDomainPagesPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the Domain Pages Summary endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiDomainPagesSummaryPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of elements within internal arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// Which backlinks to include: `all`, `live`, or `lost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_status_type: Option<String>,
    /// Result filtering conditions (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules, e.g. `["rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Filter applied to the underlying backlinks by any backlink field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_filters: Option<Vec<Value>>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Whether to include indirect (redirect/canonical) links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_indirect_links: Option<bool>,
    /// Whether to exclude backlinks from the target's own subdomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_internal_backlinks: Option<bool>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiDomainPagesSummaryPost {
    /// Builds a request for the given target (domain or subdomain).
    pub fn new(target: &str) -> BacklinksApiDomainPagesSummaryPost {
        BacklinksApiDomainPagesSummaryPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the Referring Domains endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiReferringDomainsPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of elements within internal arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// Which backlinks to include: `all`, `live`, or `lost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_status_type: Option<String>,
    /// Result filtering conditions (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules, e.g. `["rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Filter applied to the underlying backlinks by any backlink field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_filters: Option<Vec<Value>>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Whether to include indirect (redirect/canonical) links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_indirect_links: Option<bool>,
    /// Whether to exclude backlinks from the target's own subdomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_internal_backlinks: Option<bool>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiReferringDomainsPost {
    /// Builds a request for the given target.
    pub fn new(target: &str) -> BacklinksApiReferringDomainsPost {
        BacklinksApiReferringDomainsPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the Referring Networks endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiReferringNetworksPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Type of network address to group by: `ip` or `subnet`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_address_type: Option<String>,
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of elements within internal arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// Which backlinks to include: `all`, `live`, or `lost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_status_type: Option<String>,
    /// Result filtering conditions (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules, e.g. `["rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Filter applied to the underlying backlinks by any backlink field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_filters: Option<Vec<Value>>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Whether to include indirect (redirect/canonical) links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_indirect_links: Option<bool>,
    /// Whether to exclude backlinks from the target's own subdomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_internal_backlinks: Option<bool>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiReferringNetworksPost {
    /// Builds a request for the given target.
    pub fn new(target: &str) -> BacklinksApiReferringNetworksPost {
        BacklinksApiReferringNetworksPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the Competitors endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiCompetitorsPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Result filtering conditions (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules, e.g. `["rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Whether to consider only the main domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_domain: Option<bool>,
    /// Whether to omit very large domains (e.g. google.com).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_large_domains: Option<bool>,
    /// Whether to exclude backlinks from the target's own subdomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_internal_backlinks: Option<bool>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiCompetitorsPost {
    /// Builds a request for the given target.
    pub fn new(target: &str) -> BacklinksApiCompetitorsPost {
        BacklinksApiCompetitorsPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the Domain Intersection endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiDomainIntersectionPost {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Value,
    /// Targets to exclude from the results (max 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_targets: Option<Vec<String>>,
    /// Result filtering conditions (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules, e.g. `["rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Maximum number of elements within internal arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// Which backlinks to include: `all`, `live`, or `lost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_status_type: Option<String>,
    /// Filter applied to the underlying backlinks by any backlink field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_filters: Option<Vec<Value>>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Whether to include indirect (redirect/canonical) links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_indirect_links: Option<bool>,
    /// Whether to exclude backlinks from the target's own subdomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_internal_backlinks: Option<bool>,
    /// Intersection mode: `all` or `partial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersection_mode: Option<String>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiDomainIntersectionPost {
    /// Builds a request from a `targets` object keyed by id (e.g. `{"1": "domain.com"}`).
    pub fn new(targets: Value) -> BacklinksApiDomainIntersectionPost {
        BacklinksApiDomainIntersectionPost {
            targets,
            ..Default::default()
        }
    }
}

/// Request for the Page Intersection endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiPageIntersectionPost {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Value,
    /// Targets to exclude from the results (max 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_targets: Option<Vec<String>>,
    /// Which backlinks to include: `all`, `live`, or `lost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks_status_type: Option<String>,
    /// Result filtering conditions (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules, e.g. `["rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Maximum number of elements within internal arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Whether to include indirect (redirect/canonical) links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_indirect_links: Option<bool>,
    /// Whether to exclude backlinks from the target's own subdomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_internal_backlinks: Option<bool>,
    /// Intersection mode: `all` or `partial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersection_mode: Option<String>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiPageIntersectionPost {
    /// Builds a request from a `targets` object keyed by id (e.g. `{"1": "https://domain.com/"}`).
    pub fn new(targets: Value) -> BacklinksApiPageIntersectionPost {
        BacklinksApiPageIntersectionPost {
            targets,
            ..Default::default()
        }
    }
}

/// Request for the Timeseries Summary endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiTimeseriesSummaryPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Start date of the range (`yyyy-mm-dd`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End date of the range (`yyyy-mm-dd`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Grouping interval: `day`, `week`, `month`, or `year`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_range: Option<String>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiTimeseriesSummaryPost {
    /// Builds a request for the given target.
    pub fn new(target: &str) -> BacklinksApiTimeseriesSummaryPost {
        BacklinksApiTimeseriesSummaryPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the Timeseries New & Lost Summary endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiTimeseriesNewLostSummaryPost {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: String,
    /// Start date of the range (`yyyy-mm-dd`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End date of the range (`yyyy-mm-dd`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Grouping interval: `day`, `week`, `month`, or `year`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_range: Option<String>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiTimeseriesNewLostSummaryPost {
    /// Builds a request for the given target.
    pub fn new(target: &str) -> BacklinksApiTimeseriesNewLostSummaryPost {
        BacklinksApiTimeseriesNewLostSummaryPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}

/// Request for the Bulk Ranks endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkRanksPost {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Vec<String>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiBulkRanksPost {
    /// Builds a request for the given targets (up to 1000).
    pub fn new(targets: Vec<String>) -> BacklinksApiBulkRanksPost {
        BacklinksApiBulkRanksPost {
            targets,
            ..Default::default()
        }
    }
}

/// Request for the Bulk Backlinks endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkBacklinksPost {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Vec<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiBulkBacklinksPost {
    /// Builds a request for the given targets (up to 1000).
    pub fn new(targets: Vec<String>) -> BacklinksApiBulkBacklinksPost {
        BacklinksApiBulkBacklinksPost {
            targets,
            ..Default::default()
        }
    }
}

/// Request for the Bulk Spam Score endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkSpamScorePost {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Vec<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiBulkSpamScorePost {
    /// Builds a request for the given targets (up to 1000).
    pub fn new(targets: Vec<String>) -> BacklinksApiBulkSpamScorePost {
        BacklinksApiBulkSpamScorePost {
            targets,
            ..Default::default()
        }
    }
}

/// Request for the Bulk New & Lost Backlinks endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkNewLostBacklinksPost {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Vec<String>,
    /// Start date of the range (`yyyy-mm-dd`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiBulkNewLostBacklinksPost {
    /// Builds a request for the given targets (up to 1000).
    pub fn new(targets: Vec<String>) -> BacklinksApiBulkNewLostBacklinksPost {
        BacklinksApiBulkNewLostBacklinksPost {
            targets,
            ..Default::default()
        }
    }
}

/// Request for the Bulk New & Lost Referring Domains endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkNewLostReferringDomainsPost {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Vec<String>,
    /// Start date of the range (`yyyy-mm-dd`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiBulkNewLostReferringDomainsPost {
    /// Builds a request for the given targets (up to 1000).
    pub fn new(targets: Vec<String>) -> BacklinksApiBulkNewLostReferringDomainsPost {
        BacklinksApiBulkNewLostReferringDomainsPost {
            targets,
            ..Default::default()
        }
    }
}

/// Request for the Bulk Referring Domains endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkReferringDomainsPost {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Vec<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiBulkReferringDomainsPost {
    /// Builds a request for the given targets (up to 1000).
    pub fn new(targets: Vec<String>) -> BacklinksApiBulkReferringDomainsPost {
        BacklinksApiBulkReferringDomainsPost {
            targets,
            ..Default::default()
        }
    }
}

/// Request for the Bulk Pages Summary endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkPagesSummaryPost {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Vec<String>,
    /// Whether to include the target's subdomains in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Scale for rank values: `one_hundred` (0-100) or `one_thousand` (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_scale: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BacklinksApiBulkPagesSummaryPost {
    /// Builds a request for the given targets (up to 1000, max 100 different domains).
    pub fn new(targets: Vec<String>) -> BacklinksApiBulkPagesSummaryPost {
        BacklinksApiBulkPagesSummaryPost {
            targets,
            ..Default::default()
        }
    }
}

/// Request for the Errors endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiErrorsPost {
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Restrict results to errors of the given API function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_function: Option<String>,
    /// Start of the datetime range (UTC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_from: Option<String>,
    /// End of the datetime range (UTC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_to: Option<String>,
}

impl BacklinksApiErrorsPost {
    /// Builds an errors request with all fields unset.
    pub fn new() -> BacklinksApiErrorsPost {
        BacklinksApiErrorsPost::default()
    }
}

/// Request for the ID List endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiIdListPost {
    /// Start of the datetime range (UTC).
    pub datetime_from: String,
    /// End of the datetime range (UTC).
    pub datetime_to: String,
    /// Maximum number of returned elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Sort order by task execution time: `asc` or `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Whether to include task metadata in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
}

impl BacklinksApiIdListPost {
    /// Builds a request for the given UTC datetime range (`yyyy-mm-dd hh-mm-ss +00:00`).
    pub fn new(datetime_from: &str, datetime_to: &str) -> BacklinksApiIdListPost {
        BacklinksApiIdListPost {
            datetime_from: datetime_from.to_string(),
            datetime_to: datetime_to.to_string(),
            ..Default::default()
        }
    }
}
