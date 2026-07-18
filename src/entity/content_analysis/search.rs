use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of `content_analysis/search/live`: citations found for a keyword.
/// See <https://docs.dataforseo.com/v3/content_analysis/search/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiSearch {
    /// Token to pass back as `offset_token` to page beyond 10,000 results.
    pub offset_token: Option<String>,
    /// Total number of matching citations in the database.
    pub total_count: Option<i64>,
    /// Number of citations returned in `items`.
    pub items_count: Option<i64>,
    /// Citations found for the keyword.
    pub items: Option<Vec<ContentAnalysisApiSearchItem>>,
}

/// A single citation (`content_analysis_search` element).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiSearchItem {
    /// Element type, always `content_analysis_search`.
    #[serde(rename = "type")]
    pub element_type: Option<String>,
    /// URL where the citation was found.
    pub url: Option<String>,
    /// Domain name.
    pub domain: Option<String>,
    /// Main domain.
    pub main_domain: Option<String>,
    /// Rank of the URL (0–100 or 0–1000 per `rank_scale`), from the DataForSEO
    /// Backlink Index.
    pub url_rank: Option<i64>,
    /// Backlink spam score of the URL, from the DataForSEO Backlink Index.
    pub spam_score: Option<i64>,
    /// Rank of the domain (0–100 or 0–1000 per `rank_scale`), from the
    /// DataForSEO Backlink Index.
    pub domain_rank: Option<i64>,
    /// UTC datetime the crawler last visited the page
    /// (`yyyy-mm-dd hh-mm-ss +00:00`).
    pub fetch_time: Option<String>,
    /// ISO country code of the domain registration.
    pub country: Option<String>,
    /// Main language of the domain.
    pub language: Option<String>,
    /// Citation prominence score; higher means a more valuable citation.
    pub score: Option<f64>,
    /// Product/service category IDs relevant to the page.
    pub page_category: Option<Vec<i64>>,
    /// Page type classifications (e.g. blogs, news, ecommerce).
    pub page_types: Option<Vec<String>>,
    /// Ratings found on the page via microdata.
    pub ratings: Option<Vec<ContentAnalysisRating>>,
    /// Social media engagement metrics for the page.
    pub social_metrics: Option<Vec<ContentAnalysisSocialMetric>>,
    /// Detailed content data for the citation.
    pub content_info: Option<ContentAnalysisContentInfo>,
}

/// Microdata-based rating attached to a citation. Values are strings as
/// returned by the API.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisRating {
    /// Rating name (e.g. `Max5`, `Percents`, `CustomMax`).
    pub name: Option<String>,
    /// The rating value.
    pub rating_value: Option<String>,
    /// Number of votes.
    pub rating_count: Option<String>,
    /// Maximum value for the rating name.
    pub max_rating_value: Option<String>,
    /// Rating normalized relative to its maximum.
    pub relative_rating: Option<String>,
}

/// Social engagement metric for a citation.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisSocialMetric {
    /// Social platform the metric comes from.
    #[serde(rename = "type")]
    pub element_type: Option<String>,
    /// Number of likes.
    pub like_count: Option<i64>,
}

/// Detailed content data for a citation.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisContentInfo {
    /// Content classification (e.g. `page_content`, `comment`).
    pub content_type: Option<String>,
    /// Citation title.
    pub title: Option<String>,
    /// Page title.
    pub main_title: Option<String>,
    /// Title of the preceding content block.
    pub previous_title: Option<String>,
    /// Heading level of the title, 1 (top) to 6 (bottom).
    pub level: Option<i64>,
    /// Content author.
    pub author: Option<String>,
    /// Content excerpt.
    pub snippet: Option<String>,
    /// Character length of the snippet.
    pub snippet_length: Option<i64>,
    /// Social media engagement metrics for the content.
    pub social_metrics: Option<Vec<ContentAnalysisSocialMetric>>,
    /// Highlighted text from the snippet.
    pub highlighted_text: Option<String>,
    /// Content language.
    pub language: Option<String>,
    /// Probability index per sentiment connotation (anger, happiness, love,
    /// sadness, share, fun).
    pub sentiment_connotations: Option<HashMap<String, f64>>,
    /// Probability index per sentiment polarity (positive, negative, neutral).
    pub connotation_types: Option<HashMap<String, f64>>,
    /// Text category IDs of the content.
    pub text_category: Option<Vec<i64>>,
    /// UTC datetime the content was published
    /// (`yyyy-mm-dd hh-mm-ss +00:00`).
    pub date_published: Option<String>,
    /// Quality score derived from word, sentence, and character counts.
    pub content_quality_score: Option<i64>,
    /// HTML semantic element the citation sits in (e.g. `article`, `header`).
    pub semantic_location: Option<String>,
    /// Content rating related to this citation.
    pub rating: Option<ContentAnalysisRating>,
    /// UTC datetime used to group citations by date — publication date or first
    /// crawl (`yyyy-mm-dd hh-mm-ss +00:00`).
    pub group_date: Option<String>,
}
