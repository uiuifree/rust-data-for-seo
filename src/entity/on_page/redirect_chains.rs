use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};

/// Redirect chains detected among the crawled pages.
/// See <https://docs.dataforseo.com/v3/on_page/redirect_chains/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiRedirectChains {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Total number of matching items in the database.
    pub total_items_count: Option<i64>,
    /// Number of items returned in this response.
    pub items_count: Option<i64>,
    /// Redirect chain items returned in this response.
    pub items: Option<Vec<OnPageDataApiRedirectChainItem>>,
}

/// A single redirect chain, i.e. a sequence of redirecting links.
/// See <https://docs.dataforseo.com/v3/on_page/redirect_chains/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiRedirectChainItem {
    /// True when the chain forms an endless redirect loop.
    pub is_redirect_loop: Option<bool>,
    /// Ordered links that make up the redirect chain.
    pub chain: Option<Vec<OnPageDataApiRedirectChainLink>>,
}

/// A single hop within a redirect chain.
/// See <https://docs.dataforseo.com/v3/on_page/redirect_chains/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiRedirectChainLink {
    /// Link classification: "anchor", "redirect", "canonical", etc.
    #[serde(rename = "type")]
    pub link_type: Option<String>,
    /// Referring domain where the link originates.
    pub domain_from: Option<String>,
    /// Referenced domain the link points to.
    pub domain_to: Option<String>,
    /// Relative URL of the page on which the link was found.
    pub page_from: Option<String>,
    /// Relative URL of the page the link points to.
    pub page_to: Option<String>,
    /// Absolute URL of the page on which the link was found.
    pub link_from: Option<String>,
    /// Absolute URL of the page the link points to.
    pub link_to: Option<String>,
    /// True when the link is not marked nofollow.
    pub dofollow: Option<bool>,
    /// URL scheme of the referring page: "http" or "https".
    pub page_from_scheme: Option<String>,
    /// URL scheme of the referenced page: "http" or "https".
    pub page_to_scheme: Option<String>,
    /// Link scope: "internal" or "external".
    pub direction: Option<String>,
    /// True when the link points to a broken or unreachable resource.
    pub is_broken: Option<bool>,
    /// True when nofollow and dofollow links to the same target conflict.
    pub is_link_relation_conflict: Option<bool>,
}
