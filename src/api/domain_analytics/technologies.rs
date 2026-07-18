use crate::api::domain_analytics::DomainAnalyticsApi;
use crate::entity::{
    DomainAnalyticsApiAggregationTechnologies, DomainAnalyticsApiAvailableTechnologies,
    DomainAnalyticsApiDomainTechnologyItem, DomainAnalyticsApiDomainsByHtmlTerms,
    DomainAnalyticsApiDomainsByTechnology, DomainAnalyticsApiTechnologiesSummary,
    DomainAnalyticsApiTechnologyStats,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl DomainAnalyticsApi<'_> {
    /// Returns the Technologies sub-API builder.
    pub fn technologies(&self) -> DomainAnalyticsApiTechnologies<'_> {
        DomainAnalyticsApiTechnologies {
            client: self.client,
        }
    }
}

/// Builder for the [Domain Analytics Technologies](https://docs.dataforseo.com/v3/domain_analytics/technologies/overview/) endpoints.
pub struct DomainAnalyticsApiTechnologies<'a> {
    client: &'a DataForSeoClient,
}

impl DomainAnalyticsApiTechnologies<'_> {
    /// Returns the full catalogue of detectable technologies grouped by group and category.
    /// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/technologies/>.
    pub async fn available_technologies(
        &self,
    ) -> DataForSeoApiResponse<DomainAnalyticsApiAvailableTechnologies> {
        self.client
            .http_get("/v3/domain_analytics/technologies/technologies")
            .await
    }

    /// Returns domain counts aggregated by technology group, category, and technology.
    /// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/aggregation_technologies/live/>.
    pub async fn aggregation_technologies(
        &self,
        data: Vec<DomainAnalyticsApiAggregationTechnologiesPost>,
    ) -> DataForSeoApiResponse<DomainAnalyticsApiAggregationTechnologies> {
        self.client
            .http_post(
                "/v3/domain_analytics/technologies/aggregation_technologies/live",
                &data,
            )
            .await
    }

    /// Returns a distribution of matching domains across countries, languages, and keywords.
    /// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/technologies_summary/live/>.
    pub async fn technologies_summary(
        &self,
        data: Vec<DomainAnalyticsApiTechnologiesSummaryPost>,
    ) -> DataForSeoApiResponse<DomainAnalyticsApiTechnologiesSummary> {
        self.client
            .http_post(
                "/v3/domain_analytics/technologies/technologies_summary/live",
                &data,
            )
            .await
    }

    /// Returns the historical adoption of a single technology over time.
    /// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/technology_stats/live/>.
    pub async fn technology_stats(
        &self,
        data: Vec<DomainAnalyticsApiTechnologyStatsPost>,
    ) -> DataForSeoApiResponse<DomainAnalyticsApiTechnologyStats> {
        self.client
            .http_post(
                "/v3/domain_analytics/technologies/technology_stats/live",
                &data,
            )
            .await
    }

    /// Returns the list of domains using the requested technologies.
    /// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/domains_by_technology/live/>.
    pub async fn domains_by_technology(
        &self,
        data: Vec<DomainAnalyticsApiDomainsByTechnologyPost>,
    ) -> DataForSeoApiResponse<DomainAnalyticsApiDomainsByTechnology> {
        self.client
            .http_post(
                "/v3/domain_analytics/technologies/domains_by_technology/live",
                &data,
            )
            .await
    }

    /// Returns the list of domains whose HTML contains the requested terms.
    /// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/domains_by_html_terms/live/>.
    pub async fn domains_by_html_terms(
        &self,
        data: Vec<DomainAnalyticsApiDomainsByHtmlTermsPost>,
    ) -> DataForSeoApiResponse<DomainAnalyticsApiDomainsByHtmlTerms> {
        self.client
            .http_post(
                "/v3/domain_analytics/technologies/domains_by_html_terms/live",
                &data,
            )
            .await
    }

    /// Returns the technologies detected for a single target domain.
    /// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/domain_technologies/live/>.
    pub async fn domain_technologies(
        &self,
        data: Vec<DomainAnalyticsApiDomainTechnologiesPost>,
    ) -> DataForSeoApiResponse<DomainAnalyticsApiDomainTechnologyItem> {
        self.client
            .http_post(
                "/v3/domain_analytics/technologies/domain_technologies/live",
                &data,
            )
            .await
    }
}

/// A target technology path (`$group_id.$category_id`) with an optional display name.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiTechnologyPath {
    /// Technology path in the form `$group_id.$category_id`.
    pub path: String,
    /// Optional display name of the technology.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DomainAnalyticsApiTechnologyPath {
    /// Creates a technology path targeting `$group_id.$category_id`.
    pub fn new(path: String) -> DomainAnalyticsApiTechnologyPath {
        DomainAnalyticsApiTechnologyPath {
            path,
            ..Default::default()
        }
    }
}

/// Request body for the `aggregation_technologies` endpoint.
///
/// At least one of `group`, `category`, `technology`, or `keyword` must be set.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiAggregationTechnologiesPost {
    /// Technology group ID to aggregate on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Technology category ID to aggregate on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Technology name to aggregate on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology: Option<String>,
    /// Domain meta keyword to aggregate on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// Match mode: `as_is` (exact, default) or `entry` (partial).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Result filters, each a `[field, operator, value]` array (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules such as `["technologies_count,desc"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of groups returned (default 5, max 10000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_groups_list_limit: Option<i32>,
    /// Maximum number of categories per group (default 5, max 10000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_categories_list_limit: Option<i32>,
    /// Maximum number of technologies per category (default 10, max 10000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_technologies_list_limit: Option<i32>,
    /// Maximum number of items with identical group/category/technology (default 10, max 10000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// Maximum number of returned technologies (default 100, max 10000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset in the results array (default 0, max 9999).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DomainAnalyticsApiAggregationTechnologiesPost {
    /// Creates an empty request. Set one of `group`/`category`/`technology`/`keyword`.
    pub fn new() -> DomainAnalyticsApiAggregationTechnologiesPost {
        DomainAnalyticsApiAggregationTechnologiesPost::default()
    }
}

/// Request body for the `technologies_summary` endpoint.
///
/// At least one of `technology_paths`, `groups`, `categories`, `technologies`,
/// or `keywords` must be set.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiTechnologiesSummaryPost {
    /// Target technology paths (`$group_id.$category_id`), max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology_paths: Option<Vec<DomainAnalyticsApiTechnologyPath>>,
    /// Target technology group IDs, max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// Target technology category IDs, max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// Target technology names, max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technologies: Option<Vec<String>>,
    /// Domain meta keywords to match, max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Match mode: `as_is` (exact, default) or `entry` (partial).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Result filters, each a `[field, operator, value]` array (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Maximum elements in internal arrays (default 10, max 10000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DomainAnalyticsApiTechnologiesSummaryPost {
    /// Creates an empty request. Set one of the technology or keyword selectors.
    pub fn new() -> DomainAnalyticsApiTechnologiesSummaryPost {
        DomainAnalyticsApiTechnologiesSummaryPost::default()
    }
}

/// Request body for the `technology_stats` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiTechnologyStatsPost {
    /// Target technology name (e.g. `WordPress`).
    pub technology: String,
    /// Start date of the range (`yyyy-mm-dd`, minimum `2022-10-31`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End date of the range (`yyyy-mm-dd`, defaults to today).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DomainAnalyticsApiTechnologyStatsPost {
    /// Creates a request for the given technology name.
    pub fn new(technology: String) -> DomainAnalyticsApiTechnologyStatsPost {
        DomainAnalyticsApiTechnologyStatsPost {
            technology,
            ..Default::default()
        }
    }
}

/// Request body for the `domains_by_technology` endpoint.
///
/// At least one of `technologies`, `technology_paths`, `groups`, `categories`,
/// or `keywords` must be set.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiDomainsByTechnologyPost {
    /// Target technology names, max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technologies: Option<Vec<String>>,
    /// Target technology paths (`$group_id.$category_id`), max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology_paths: Option<Vec<DomainAnalyticsApiTechnologyPath>>,
    /// Target technology group IDs, max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// Target technology category IDs, max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// Domain meta keywords to match, max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Match mode: `as_is` (exact, default) or `entry` (partial).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Result filters, each a `[field, operator, value]` array (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules such as `["domain_rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned domains (default 100, max 10000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset in the results array (default 0, max 9999).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Pagination token for fetching results beyond the offset limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DomainAnalyticsApiDomainsByTechnologyPost {
    /// Creates a request for the given technology names.
    pub fn new(technologies: Vec<String>) -> DomainAnalyticsApiDomainsByTechnologyPost {
        DomainAnalyticsApiDomainsByTechnologyPost {
            technologies: Some(technologies),
            ..Default::default()
        }
    }
}

/// Request body for the `domains_by_html_terms` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiDomainsByHtmlTermsPost {
    /// HTML elements, tags, or attributes to search for, max 10.
    pub search_terms: Vec<String>,
    /// Domain meta keywords to match, max 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Match mode: `entry` (default) or `strict_entry`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Result filters, each a `[field, operator, value]` array (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Sorting rules such as `["domain_rank,desc"]` (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Maximum number of returned domains (default 100, max 10000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset in the results array (default 0, max 9999).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Pagination token for fetching results beyond the offset limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DomainAnalyticsApiDomainsByHtmlTermsPost {
    /// Creates a request for the given HTML search terms (max 10).
    pub fn new(search_terms: Vec<String>) -> DomainAnalyticsApiDomainsByHtmlTermsPost {
        DomainAnalyticsApiDomainsByHtmlTermsPost {
            search_terms,
            ..Default::default()
        }
    }
}

/// Request body for the `domain_technologies` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiDomainTechnologiesPost {
    /// Target domain name to analyze.
    pub target: String,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DomainAnalyticsApiDomainTechnologiesPost {
    /// Creates a request for the given target domain.
    pub fn new(target: String) -> DomainAnalyticsApiDomainTechnologiesPost {
        DomainAnalyticsApiDomainTechnologiesPost {
            target,
            ..Default::default()
        }
    }
}
