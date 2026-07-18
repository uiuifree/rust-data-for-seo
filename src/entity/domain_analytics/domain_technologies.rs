use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A single domain profile enriched with detected technologies.
///
/// Shared by the `domain_technologies`, `domains_by_technology`, and
/// `domains_by_html_terms` endpoints.
/// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/domain_technologies/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiDomainTechnologyItem {
    /// Result element type, e.g. `domain_technology_item`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Domain name the profile describes.
    pub domain: Option<String>,
    /// Meta title of the domain's homepage.
    pub title: Option<String>,
    /// Meta description of the domain's homepage.
    pub description: Option<String>,
    /// Meta keywords declared on the domain's homepage.
    pub meta_keywords: Option<Vec<String>>,
    /// Domain rank based on the DataForSEO backlink index (0-1000).
    pub domain_rank: Option<i64>,
    /// Date the domain was last crawled (UTC).
    pub last_visited: Option<String>,
    /// ISO country code detected for the domain.
    pub country_iso_code: Option<String>,
    /// Language code detected for the domain.
    pub language_code: Option<String>,
    /// Language code declared for the domain's content.
    pub content_language_code: Option<String>,
    /// Phone numbers found on the domain.
    pub phone_numbers: Option<Vec<String>>,
    /// Email addresses found on the domain.
    pub emails: Option<Vec<String>>,
    /// Social media profile URLs found on the domain.
    pub social_graph_urls: Option<Vec<String>>,
    /// Detected technologies grouped by technology group and category.
    /// The structure is a dynamic hierarchy, so it is exposed as raw JSON.
    pub technologies: Option<Value>,
}
