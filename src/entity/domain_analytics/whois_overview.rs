use serde::{Deserialize, Serialize};

/// Result object for the Whois `overview` endpoint.
/// See <https://docs.dataforseo.com/v3/domain_analytics/whois/overview/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiWhoisOverview {
    /// Total number of domains matching the request across all pages.
    pub total_count: Option<i64>,
    /// Number of domain records returned in this response.
    pub items_count: Option<i64>,
    /// Offset applied to the returned results.
    pub offset: Option<i32>,
    /// Pagination token to fetch the next batch of results.
    pub offset_token: Option<String>,
    /// The returned Whois records.
    pub items: Option<Vec<DomainAnalyticsApiWhoisItem>>,
}

/// Whois record for a single domain enriched with ranking and backlink stats.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiWhoisItem {
    /// Result element type, e.g. `domain_whois_overview`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Domain name the record describes.
    pub domain: Option<String>,
    /// Domain registration date (UTC).
    pub created_datetime: Option<String>,
    /// Date the domain's Whois record was last changed (UTC).
    pub changed_datetime: Option<String>,
    /// Date the domain registration expires (UTC).
    pub expiration_datetime: Option<String>,
    /// Date DataForSEO last updated this record (UTC).
    pub updated_datetime: Option<String>,
    /// Date the domain was first seen in the DataForSEO database (UTC).
    pub first_seen: Option<String>,
    /// EPP status codes currently set on the domain.
    pub epp_status_codes: Option<Vec<String>>,
    /// Top-level domain of the domain name.
    pub tld: Option<String>,
    /// Whether the domain is currently registered.
    pub registered: Option<bool>,
    /// Name of the domain's registrar.
    pub registrar: Option<String>,
    /// Organic and paid SERP metrics for the domain.
    pub metrics: Option<DomainAnalyticsApiWhoisMetrics>,
    /// Backlink profile summary for the domain.
    pub backlinks_info: Option<DomainAnalyticsApiWhoisBacklinksInfo>,
}

/// Organic and paid SERP metrics attached to a Whois record.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiWhoisMetrics {
    /// Metrics for keywords the domain ranks for organically.
    pub organic: Option<DomainAnalyticsApiWhoisMetricsInfo>,
    /// Metrics for keywords the domain ranks for through paid ads.
    pub paid: Option<DomainAnalyticsApiWhoisMetricsInfo>,
}

/// Keyword-position distribution and traffic estimates for a domain.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiWhoisMetricsInfo {
    /// Number of keywords ranking in position 1.
    pub pos_1: Option<i64>,
    /// Number of keywords ranking in positions 2-3.
    pub pos_2_3: Option<i64>,
    /// Number of keywords ranking in positions 4-10.
    pub pos_4_10: Option<i64>,
    /// Number of keywords ranking in positions 11-20.
    pub pos_11_20: Option<i64>,
    /// Number of keywords ranking in positions 21-30.
    pub pos_21_30: Option<i64>,
    /// Number of keywords ranking in positions 31-40.
    pub pos_31_40: Option<i64>,
    /// Number of keywords ranking in positions 41-50.
    pub pos_41_50: Option<i64>,
    /// Number of keywords ranking in positions 51-60.
    pub pos_51_60: Option<i64>,
    /// Number of keywords ranking in positions 61-70.
    pub pos_61_70: Option<i64>,
    /// Number of keywords ranking in positions 71-80.
    pub pos_71_80: Option<i64>,
    /// Number of keywords ranking in positions 81-90.
    pub pos_81_90: Option<i64>,
    /// Number of keywords ranking in positions 91-100.
    pub pos_91_100: Option<i64>,
    /// Estimated monthly traffic volume derived from ranking position and CTR.
    pub etv: Option<f64>,
    /// Total number of ranking keywords.
    pub count: Option<i64>,
    /// Estimated monthly cost (USD) of the equivalent paid traffic.
    pub estimated_paid_traffic_cost: Option<f64>,
}

/// Backlink summary attached to a Whois record.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiWhoisBacklinksInfo {
    /// Number of referring domains pointing to the target.
    pub referring_domains: Option<i64>,
    /// Number of referring root domains pointing to the target.
    pub referring_main_domains: Option<i64>,
    /// Number of referring pages pointing to the target.
    pub referring_pages: Option<i64>,
    /// Number of dofollow backlinks pointing to the target.
    pub dofollow: Option<i64>,
    /// Total number of backlinks pointing to the target.
    pub backlinks: Option<i64>,
    /// Date the backlink metrics were last updated (UTC).
    pub time_update: Option<String>,
}
