use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceHtml {
    pub resource_type: Option<String>,
    pub status_code: Option<i32>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub meta: Option<OnPageResourceMeta>,
    pub page_timing: Option<OnPageResourceHtmlPageTiming>,
    pub onpage_score: Option<f32>,
    pub total_dom_size: Option<i32>,
    pub custom_js_response: Option<Value>,
    pub custom_js_client_exception: Option<String>,
    pub broken_resources: Option<bool>,
    pub broken_links: Option<bool>,
    pub duplicate_title: Option<bool>,
    pub duplicate_description: Option<bool>,
    pub duplicate_content: Option<bool>,
    pub click_depth: Option<i32>,
    pub size: Option<i32>,
    pub encoded_size: Option<i32>,
    pub total_transfer_size: Option<i32>,
    pub fetch_time: Option<String>,
    pub cache_control: Option<OnPageResourceHtmlCacheControl>,
    pub checks: Option<OnPageResourceHtmlWebsiteChecks>,
    pub content_encoding: Option<String>,
    pub media_type: Option<String>,
    pub server: Option<String>,
    pub is_resource: Option<bool>,
    pub last_modified: Option<OnPageResourceHtmlLastModified>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceMeta {
    pub title: Option<String>,
    pub charset: Option<i32>,
    pub follow: Option<bool>,
    pub generator: Option<String>,
    pub htags: Option<HashMap<String, Option<Vec<String>>>>,
    pub description: Option<String>,
    pub favicon: Option<String>,
    pub meta_keywords: Option<String>,
    pub canonical: Option<String>,
    pub internal_links_count: Option<i32>,
    pub external_links_count: Option<i32>,
    pub images_count: Option<i32>,
    pub images_size: Option<i32>,
    pub scripts_count: Option<i32>,
    pub scripts_size: Option<i32>,
    pub stylesheets_count: Option<i32>,
    pub stylesheets_size: Option<i32>,
    pub title_length: Option<i32>,
    pub description_length: Option<i32>,
    pub render_blocking_scripts_count: Option<i32>,
    pub render_blocking_stylesheets_count: Option<i32>,
    pub cumulative_layout_shift: Option<f32>,
    pub social_media_tags: Option<Value>,
    pub content: Option<OnPageResourceMetaContent>,
    pub deprecated_tags: Option<Vec<String>>,
    pub duplicate_meta_tags: Option<Vec<String>>,
    pub spell: Option<OnPageSpell>,
    pub resource_errors: Option<OnPageResourceErrors>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceMetaContent {
    pub plain_text_size: Option<i32>,
    pub plain_text_rate: Option<f32>,
    pub plain_text_word_count: Option<f32>,
    pub automated_readability_index: Option<f32>,
    pub coleman_liau_readability_index: Option<f32>,
    pub dale_chall_readability_index: Option<f32>,
    pub flesch_kincaid_readability_index: Option<f32>,
    pub smog_readability_index: Option<f32>,
    pub description_to_content_consistency: Option<f32>,
    pub title_to_content_consistency: Option<f32>,
    pub meta_keywords_to_content_consistency: Option<f32>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageSpell {
    pub hunspell_language_code: Option<String>,
    pub misspelled: Option<Vec<OnPageSpellMisspelled>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageSpellMisspelled {
    pub word: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceErrors {
    pub errors: Option<Vec<OnPageResourceErrorsError>>,
    pub warnings: Option<Vec<OnPageResourceErrorsWarning>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceErrorsError {
    pub line: Option<i32>,
    pub message: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceErrorsWarning {
    pub line: Option<i32>,
    pub message: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceHtmlPageTiming {
    pub time_to_interactive: Option<i32>,
    pub dom_complete: Option<i32>,
    pub largest_contentful_paint: Option<f32>,
    pub first_input_delay: Option<f32>,
    pub connection_time: Option<i32>,
    pub time_to_secure_connection: Option<i32>,
    pub request_sent_time: Option<i32>,
    pub waiting_time: Option<i32>,
    pub download_time: Option<i32>,
    pub duration_time: Option<i32>,
    pub fetch_start: Option<i32>,
    pub fetch_end: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceHtmlCacheControl {
    pub cachable: Option<bool>,
    pub ttl: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceHtmlWebsiteChecks {
    pub no_content_encoding: Option<bool>,
    pub high_loading_time: Option<bool>,
    pub is_redirect: Option<bool>,
    pub is_4xx_code: Option<bool>,
    pub is_5xx_code: Option<bool>,
    pub is_broken: Option<bool>,
    pub is_www: Option<bool>,
    pub is_https: Option<bool>,
    pub is_http: Option<bool>,
    pub high_waiting_time: Option<bool>,
    pub no_doctype: Option<bool>,
    pub canonical: Option<bool>,
    pub no_encoding_meta_tag: Option<bool>,
    pub no_h1_tag: Option<bool>,
    pub https_to_http_links: Option<bool>,
    pub has_html_doctype: Option<bool>,
    pub size_greater_than_3mb: Option<bool>,
    pub meta_charset_consistency: Option<bool>,
    pub has_meta_refresh_redirect: Option<bool>,
    pub has_render_blocking_resources: Option<bool>,
    pub redirect_chain: Option<bool>,
    pub low_content_rate: Option<bool>,
    pub high_content_rate: Option<bool>,
    pub low_character_count: Option<bool>,
    pub high_character_count: Option<bool>,
    pub small_page_size: Option<bool>,
    pub large_page_size: Option<bool>,
    pub low_readability_rate: Option<bool>,
    pub irrelevant_description: Option<bool>,
    pub irrelevant_title: Option<bool>,
    pub irrelevant_meta_keywords: Option<bool>,
    pub title_too_long: Option<bool>,
    pub title_too_short: Option<bool>,
    pub deprecated_html_tags: Option<bool>,
    pub duplicate_meta_tags: Option<bool>,
    pub duplicate_title_tag: Option<bool>,
    pub no_image_alt: Option<bool>,
    pub no_image_title: Option<bool>,
    pub no_description: Option<bool>,
    pub no_title: Option<bool>,
    pub no_favicon: Option<bool>,
    pub seo_friendly_url: Option<bool>,
    pub flash: Option<bool>,
    pub frame: Option<bool>,
    pub lorem_ipsum: Option<bool>,
    pub has_misspelling: Option<bool>,
    pub seo_friendly_url_characters_check: Option<bool>,
    pub seo_friendly_url_dynamic_check: Option<bool>,
    pub seo_friendly_url_keywords_check: Option<bool>,
    pub seo_friendly_url_relative_length_check: Option<bool>,
    pub is_orphan_page: Option<bool>,
    pub is_link_relation_conflict: Option<bool>,
    pub has_links_to_redirects: Option<bool>,
    pub canonical_chain: Option<bool>,
    pub canonical_to_redirect: Option<bool>,
    pub canonical_to_broken: Option<bool>,
    pub recursive_canonical: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceHtmlLastModified {
    pub header: Option<String>,
    pub sitemap: Option<String>,
    pub meta_tag: Option<String>,
}
