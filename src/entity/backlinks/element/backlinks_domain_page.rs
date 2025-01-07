use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksDomainPage {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub main_domain: Option<String>,
    pub domain: Option<String>,
    pub tld: Option<String>,
    pub page: Option<String>,
    pub ip: Option<String>,
    pub first_visited: Option<String>,
    pub prev_visited: Option<String>,
    pub fetch_time: Option<String>,
    pub status_code: Option<i32>,
    pub location: Option<String>,
    pub size: Option<i32>,
    pub encoded_size: Option<i32>,
    pub content_encoding: Option<String>,
    pub media_type: Option<String>,
    pub server: Option<String>,
    pub meta: Option<BacklinksApiElementBacklinksDomainPageMeta>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksDomainPageMeta {
    pub title: Option<String>,
    pub canonical: Option<String>,
    pub internal_links_count: Option<i32>,
    pub external_links_count: Option<i32>,
    pub images_count: Option<i32>,
    pub words_count: Option<i32>,
    pub page_spam_score: Option<i32>,
    pub social_media_tags: Option<HashMap<String, String>>,
    pub h1: Option<String>,
    pub h2: Option<String>,
    pub h3: Option<String>,
    pub images_alt: Option<Vec<String>>,
    pub powered_by: Option<Vec<String>>,
    pub language: Option<String>,
    pub charset: Option<String>,
    pub platform_type: Option<Vec<String>>,
    pub technologies: Option<Vec<BacklinksApiElementBacklinksDomainPageWebpageTechnologies>>,
    pub page_summary: Option<Vec<BacklinksApiElementBacklinksDomainPagePageSummary>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksDomainPageWebpageTechnologies {
    pub cms: Option<String>,
    pub blogs: Option<String>,
    #[serde(rename = "web-server")]
    pub web_servers: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksDomainPagePageSummary {
    pub first_seen: Option<String>,
    pub lost_date: Option<String>,
    pub rank: Option<i32>,
    pub backlinks: Option<i32>,
    pub backlinks_spam_score: Option<i32>,
    pub broken_backlinks: Option<i32>,
    pub broken_pages: Option<i32>,
    pub referring_domains: Option<i32>,
    pub referring_domains_nofollow: Option<i32>,
    pub referring_main_domains: Option<i32>,
    pub referring_main_domains_nofollow: Option<i32>,
    pub referring_ips: Option<i32>,
    pub referring_subnets: Option<i32>,
    pub referring_pages: Option<i32>,
    pub referring_links_tld: Option<HashMap<String, i32>>,
    pub referring_links_types: Option<HashMap<String, i32>>,
    pub referring_links_attributes: Option<HashMap<String, i32>>,
    pub referring_links_platform_types: Option<HashMap<String, i32>>,
    pub referring_links_semantic_locations: Option<HashMap<String, i32>>,
    pub referring_links_countries: Option<HashMap<String, i32>>,
    pub referring_pages_nofollow: Option<i32>,
}
