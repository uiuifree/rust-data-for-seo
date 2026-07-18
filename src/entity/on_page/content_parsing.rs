use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OnPage Content Parsing result: structured content extracted from a page.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentParsing {
    /// Status of the crawling session (`in_progress`, `finished`).
    pub crawl_progress: Option<String>,
    /// Details of the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Number of items in the `items` array.
    pub items_count: Option<i64>,
    /// Array of parsed pages.
    pub items: Option<Vec<OnPageDataApiContentParsingItem>>,
}

/// A single parsed page in a Content Parsing result.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentParsingItem {
    /// Item type; `content_parsing_element`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Date and time when the content was fetched, in UTC.
    pub fetch_time: Option<String>,
    /// HTTP response code of the parsed page.
    pub status_code: Option<i32>,
    /// Structured page content organized by section.
    pub page_content: Option<OnPageDataApiPageContent>,
    /// Page content rendered as Markdown; returned only when `markdown_view` is enabled.
    pub page_as_markdown: Option<String>,
}

/// Structured page content grouped by section.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiPageContent {
    /// Content of the page header section.
    pub header: Option<OnPageDataApiContentSection>,
    /// Content of the page footer section.
    pub footer: Option<OnPageDataApiContentSection>,
    /// Main topics identified on the page.
    pub main_topic: Option<Vec<OnPageDataApiContentTopic>>,
    /// Secondary topics identified on the page.
    pub secondary_topic: Option<Vec<OnPageDataApiContentTopic>>,
    /// Rating information for products displayed on the page.
    pub ratings: Option<Vec<OnPageDataApiContentRating>>,
    /// Products displayed on the page.
    pub offers: Option<Vec<OnPageDataApiContentOffer>>,
    /// Comments displayed on the page.
    pub comments: Option<Vec<OnPageDataApiContentComment>>,
    /// Contact information displayed on the page.
    pub contacts: Option<OnPageDataApiContentContacts>,
}

/// A content section (header or footer) with its primary, secondary, and table content.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentSection {
    /// Primary content elements of the section.
    pub primary_content: Option<Vec<OnPageDataApiContentElement>>,
    /// Secondary content elements of the section.
    pub secondary_content: Option<Vec<OnPageDataApiContentElement>>,
    /// Tables found in the section.
    pub table_content: Option<Vec<OnPageDataApiContentTable>>,
}

/// A topic block on the page with its metadata and content.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentTopic {
    /// Meta title of the topic block.
    pub h_title: Option<String>,
    /// Main title of the block.
    pub main_title: Option<String>,
    /// Author of the content.
    pub author: Option<String>,
    /// Language of the content.
    pub language: Option<String>,
    /// HTML heading level of the block.
    pub level: Option<String>,
    /// Primary content elements of the topic.
    pub primary_content: Option<Vec<OnPageDataApiContentElement>>,
    /// Secondary content elements of the topic.
    pub secondary_content: Option<Vec<OnPageDataApiContentElement>>,
    /// Tables found in the topic.
    pub table_content: Option<Vec<OnPageDataApiContentTable>>,
}

/// A single content element, optionally carrying link URLs.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentElement {
    /// Text of the content element.
    pub text: Option<String>,
    /// Page URL displayed when the text is a link anchor.
    pub url: Option<String>,
    /// Additional URLs and anchors within the element.
    pub urls: Option<Vec<OnPageDataApiContentUrl>>,
}

/// A hyperlink with its anchor text.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentUrl {
    /// Link destination URL.
    pub url: Option<String>,
    /// Anchor text of the URL.
    pub anchor_text: Option<String>,
}

/// A parsed table with its header, body, and footer rows.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentTable {
    /// Header rows of the table.
    pub header: Option<Vec<Value>>,
    /// Body rows of the table.
    pub body: Option<Vec<Value>>,
    /// Footer rows of the table.
    pub footer: Option<Vec<Value>>,
}

/// Rating information for a product displayed on the page.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentRating {
    /// Rating identifier (often `null`).
    pub name: Option<String>,
    /// Numeric rating value.
    pub rating_value: Option<f64>,
    /// Maximum possible rating value.
    pub max_rating_value: Option<f64>,
    /// Number of ratings received.
    pub rating_count: Option<i64>,
    /// Normalized rating score, from 0 to 1.
    pub relative_rating: Option<f64>,
}

/// A product offer displayed on the page.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentOffer {
    /// Name of the product.
    pub name: Option<String>,
    /// Price of the product.
    pub price: Option<f64>,
    /// Currency code for the price.
    pub price_currency: Option<String>,
    /// Date and time until which the price is valid.
    pub price_valid_until: Option<String>,
}

/// A comment displayed on the page.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentComment {
    /// Rating attached to the comment.
    pub rating: Option<OnPageDataApiContentRating>,
    /// Title of the comment.
    pub title: Option<String>,
    /// Date the comment was published.
    pub publish_date: Option<String>,
    /// Author of the comment.
    pub author: Option<String>,
    /// Text of the comment, with any URLs.
    pub primary_content: Option<Vec<OnPageDataApiContentElement>>,
}

/// Contact information displayed on the page.
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiContentContacts {
    /// Telephone numbers found on the page.
    pub telephones: Option<Vec<String>>,
    /// Email addresses found on the page.
    pub emails: Option<Vec<String>>,
}
