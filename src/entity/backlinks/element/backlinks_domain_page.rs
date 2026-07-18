use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single crawled page item returned by the Domain Pages endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/domain_pages/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksDomainPage {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Whether to consider only the main domain.
    pub main_domain: Option<String>,
    /// Referring domain name.
    pub domain: Option<String>,
    /// Top-level domain of the page.
    pub tld: Option<String>,
    /// URL of the page.
    pub page: Option<String>,
    /// IP address of the page.
    pub ip: Option<String>,
    /// Date and time the page was first crawled (UTC).
    pub first_visited: Option<String>,
    /// Date and time the page was visited before `fetch_time` (UTC).
    pub prev_visited: Option<String>,
    /// Date and time the page was last crawled (UTC).
    pub fetch_time: Option<String>,
    /// HTTP status code.
    pub status_code: Option<i32>,
    /// Value of the page's redirect `Location` header.
    pub location: Option<String>,
    /// Size of the page in bytes.
    pub size: Option<i64>,
    /// Encoded size of the page in bytes.
    pub encoded_size: Option<i64>,
    /// Content encoding of the page.
    pub content_encoding: Option<String>,
    /// Media type of the page content.
    pub media_type: Option<String>,
    /// Server software reported by the target.
    pub server: Option<String>,
    /// Metadata about the page (title, headings, links, technologies).
    pub meta: Option<BacklinksApiElementBacklinksDomainPageMeta>,
}
/// Page metadata (title, headings, link counts, technologies) for a crawled page.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksDomainPageMeta {
    /// Title of the page.
    pub title: Option<String>,
    /// Canonical URL declared by the page.
    pub canonical: Option<String>,
    /// Number of internal links on the target.
    pub internal_links_count: Option<i64>,
    /// Number of external links on the target.
    pub external_links_count: Option<i64>,
    /// Number of images on the page.
    pub images_count: Option<i64>,
    /// Number of words on the page.
    pub words_count: Option<i64>,
    /// Spam score of the page (0-100).
    pub page_spam_score: Option<i64>,
    /// Social media tags found on the page with their values.
    pub social_media_tags: Option<HashMap<String, String>>,
    /// First `h1` heading on the page.
    pub h1: Option<String>,
    /// First `h2` heading on the page.
    pub h2: Option<String>,
    /// First `h3` heading on the page.
    pub h3: Option<String>,
    /// Alt texts of images on the page.
    pub images_alt: Option<Vec<String>>,
    /// `powered-by` technologies declared by the page.
    pub powered_by: Option<Vec<String>>,
    /// Language of the page.
    pub language: Option<String>,
    /// Character set of the page.
    pub charset: Option<String>,
    /// Platform types detected for the target.
    pub platform_type: Option<Vec<String>>,
    /// Technologies detected on the page.
    pub technologies: Option<Vec<BacklinksApiElementBacklinksDomainPageWebpageTechnologies>>,
    /// Backlink summary metrics for the page.
    pub page_summary: Option<Vec<BacklinksApiElementBacklinksDomainPagePageSummary>>,
}
/// Technologies detected on a crawled page.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksDomainPageWebpageTechnologies {
    /// Content management system detected for the target.
    pub cms: Option<String>,
    /// Blog engine detected on the page.
    pub blogs: Option<String>,
    /// Web server technology detected on the page.
    #[serde(rename = "web-server")]
    pub web_servers: Option<String>,
}
/// Backlink summary metrics for a crawled page.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksDomainPagePageSummary {
    /// Date and time the element was first found (UTC).
    pub first_seen: Option<String>,
    /// Date and time the element was last seen before being lost (UTC).
    pub lost_date: Option<String>,
    /// Rank of this element on the configured scale (0-1000 by default).
    pub rank: Option<i64>,
    /// Number of backlinks pointing to the target.
    pub backlinks: Option<i64>,
    /// Average spam score of the backlinks (0-100).
    pub backlinks_spam_score: Option<i64>,
    /// Number of broken backlinks.
    pub broken_backlinks: Option<i64>,
    /// Number of referring pages returning a 4xx/5xx status.
    pub broken_pages: Option<i64>,
    /// Number of referring domains pointing to the target.
    pub referring_domains: Option<i64>,
    /// Number of referring domains with at least one nofollow link.
    pub referring_domains_nofollow: Option<i64>,
    /// Number of referring main (root) domains.
    pub referring_main_domains: Option<i64>,
    /// Number of referring main domains with at least one nofollow link.
    pub referring_main_domains_nofollow: Option<i64>,
    /// Number of referring IP addresses.
    pub referring_ips: Option<i64>,
    /// Number of referring subnetworks.
    pub referring_subnets: Option<i64>,
    /// Number of referring pages pointing to the target.
    pub referring_pages: Option<i64>,
    /// Distribution of referring links by top-level domain with counts.
    pub referring_links_tld: Option<HashMap<String, i64>>,
    /// Distribution of referring link types (anchor, image, link, ...) with counts.
    pub referring_links_types: Option<HashMap<String, i64>>,
    /// Distribution of referring link attributes (nofollow, ugc, ...) with counts.
    pub referring_links_attributes: Option<HashMap<String, i64>>,
    /// Distribution of referring links by platform type with counts.
    pub referring_links_platform_types: Option<HashMap<String, i64>>,
    /// Distribution of referring links by HTML semantic location with counts.
    pub referring_links_semantic_locations: Option<HashMap<String, i64>>,
    /// Distribution of referring links by ISO country code with counts.
    pub referring_links_countries: Option<HashMap<String, i64>>,
    /// Number of referring pages with at least one nofollow link.
    pub referring_pages_nofollow: Option<i64>,
}
