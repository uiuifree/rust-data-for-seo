use serde::{Deserialize, Serialize};

/// A single backlink item returned by the Backlinks endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/backlinks/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksElementBacklink {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Domain the backlink originates from.
    pub domain_from: Option<String>,
    /// URL of the page the backlink originates from.
    pub url_from: Option<String>,
    /// Whether the `url_from` page uses HTTPS.
    pub url_from_https: Option<bool>,
    /// Domain the backlink points to.
    pub domain_to: Option<String>,
    /// URL the backlink points to.
    pub url_to: Option<String>,
    /// Whether the `url_to` page uses HTTPS.
    pub url_to_https: Option<bool>,
    /// Top-level domain of the referring page.
    pub tld_from: Option<String>,
    /// Whether the backlink is newly discovered.
    pub is_new: Option<bool>,
    /// Whether the backlink has been lost.
    pub is_lost: Option<bool>,
    /// Spam score of the backlink (0-100).
    pub backlink_spam_score: Option<i32>,
    /// Rank of this element on the configured scale (0-1000 by default).
    pub rank: Option<i32>,
    /// Rank of the referring page.
    pub page_from_rank: Option<i32>,
    /// Rank of the referring domain.
    pub domain_from_rank: Option<i32>,
    /// Platform types detected for the referring domain.
    pub domain_from_platform_type: Option<Vec<String>>,
    /// Whether the referring domain is an IP address.
    pub domain_from_is_ip: Option<bool>,
    /// IP address of the referring domain.
    pub domain_from_ip: Option<String>,
    /// Country of the referring domain.
    pub domain_from_country: Option<String>,
    /// Number of external links on the referring page.
    pub page_from_external_links: Option<i32>,
    /// Number of internal links on the referring page.
    pub page_from_internal_links: Option<i32>,
    /// Size of the referring page in bytes.
    pub page_from_size: Option<i32>,
    /// Character encoding of the referring page.
    pub page_from_encoding: Option<String>,
    /// Language of the referring page.
    pub page_from_language: Option<String>,
    /// Title of the referring page.
    pub page_from_title: Option<String>,
    /// HTTP status code of the referring page.
    pub page_from_status_code: Option<i32>,
    /// Date and time the element was first found (UTC).
    pub first_seen: Option<String>,
    /// Date and time the backlink was seen before `last_seen` (UTC).
    pub prev_seen: Option<String>,
    /// Date and time the backlink was last seen (UTC).
    pub last_seen: Option<String>,
    /// Type of the backlink (anchor, image, redirect, ...).
    pub item_type: Option<String>,
    /// HTML attributes of the backlink (nofollow, ugc, ...).
    pub attributes: Option<Vec<String>>,
    /// Whether the backlink is dofollow.
    pub dofollow: Option<bool>,
    /// Whether the backlink is still present on the referring page.
    pub original: Option<bool>,
    /// Alt text of the backlink when it is an image.
    pub alt: Option<String>,
    /// URL of the image when the backlink is an image link.
    pub image_url: Option<String>,
    /// Anchor text of the referring links.
    pub anchor: Option<String>,
    /// Text immediately preceding the anchor.
    pub text_pre: Option<String>,
    /// Text immediately following the anchor.
    pub text_post: Option<String>,
    /// HTML semantic location of the backlink on the referring page.
    pub semantic_location: Option<String>,
    /// Number of links from the referring page to the target.
    pub links_count: Option<i32>,
    /// Number of grouped backlinks represented by this item.
    pub group_count: Option<i32>,
    /// Whether the backlink is broken.
    pub is_broken: Option<bool>,
    /// HTTP status code returned by the `url_to` page.
    pub url_to_status_code: Option<i32>,
    /// Spam score of the `url_to` page (0-100).
    pub url_to_spam_score: Option<i32>,
    /// Redirect target of the `url_to` page, if any.
    pub url_to_redirect_target: Option<String>,
    /// Ranked-keyword counts for the referring page.
    pub ranked_keywords_info: Option<BacklinksElementBacklinkRankKeywordsInfo>,
    /// Whether the backlink reaches the target through a redirect or canonical chain.
    pub is_indirect_link: Option<bool>,
    /// Redirect/canonical chain followed by an indirect backlink.
    pub indirect_link_path: Option<Vec<BacklinksElementIndirectLinkPath>>,
}

/// Ranked-keyword counts for the page a backlink originates from.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksElementBacklinkRankKeywordsInfo {
    /// Number of keywords the referring page ranks for in the top 3.
    pub page_from_keywords_count_top_3: Option<i32>,
    /// Number of keywords the referring page ranks for in the top 10.
    pub page_from_keywords_count_top_10: Option<i32>,
    /// Number of keywords the referring page ranks for in the top 100.
    pub page_from_keywords_count_top_100: Option<i32>,
}

/// A single hop in the redirect/canonical chain of an indirect backlink.
/// See <https://docs.dataforseo.com/v3/backlinks/backlinks/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksElementIndirectLinkPath {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// HTTP status code returned by this hop.
    pub status_code: Option<i32>,
    /// URL of this hop in the redirect/canonical chain.
    pub url: Option<String>,
}
