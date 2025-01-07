use serde::{Deserialize, Serialize};
use crate::entity::BacklinksIndirectLinkPath;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksPageInterSection {
    #[serde(rename = "type")]
    pub domain_from: Option<i32>,
    pub url_from: Option<String>,
    pub url_from_https: Option<bool>,
    pub domain_to: Option<String>,
    pub url_to: Option<String>,
    pub url_to_https: Option<bool>,
    pub tld_from: Option<String>,
    pub is_new: Option<bool>,
    pub is_lost: Option<bool>,
    pub backlink_spam_score: Option<i32>,
    pub rank: Option<i32>,
    pub page_from_rank: Option<i32>,
    pub domain_from_rank: Option<i32>,
    pub domain_from_platform_type: Option<Vec<String>>,
    pub domain_from_is_ip: Option<bool>,
    pub domain_from_ip: Option<String>,
    pub domain_from_country: Option<String>,
    pub page_from_external_links: Option<i32>,
    pub page_from_internal_links: Option<i32>,
    pub page_from_size: Option<i32>,
    pub page_from_encoding: Option<String>,
    pub page_from_language: Option<String>,
    pub page_from_title: Option<String>,
    pub page_from_status_code: Option<i32>,
    pub first_seen: Option<String>,
    pub prev_seen: Option<String>,
    pub last_seen: Option<String>,
    pub item_type: Option<String>,
    pub attributes: Option<Vec<String>>,
    pub dofollow: Option<bool>,
    pub original: Option<bool>,
    pub alt: Option<String>,
    pub anchor: Option<String>,
    pub text_pre: Option<String>,
    pub text_post: Option<String>,
    pub semantic_location: Option<String>,
    pub links_count: Option<i32>,
    pub group_count: Option<i32>,
    pub is_broken: Option<bool>,
    pub url_to_status_code: Option<i32>,
    pub url_to_spam_score: Option<i32>,
    pub url_to_redirect_target: Option<String>,
    pub is_indirect_link: Option<bool>,
    pub indirect_link_path: Option<BacklinksIndirectLinkPath>,
}