use serde::{Deserialize, Serialize};

/// YouTube video-comments advanced result object.
/// See <https://docs.dataforseo.com/v3/serp/youtube/video_comments/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiYoutubeVideoCommentsTaskAdvanced {
    /// YouTube video identifier.
    pub video_id: Option<String>,
    /// Search-engine domain the results were taken from.
    pub se_domain: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Direct URL to reproduce the search on the search engine.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was received.
    pub datetime: Option<String>,
    /// Total number of comments on the video.
    pub comments_count: Option<i64>,
    /// Number of items returned in this result.
    pub items_count: Option<i64>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiElementYoutubeComment>>,
}

/// `youtube_comment` item: a single top-level comment.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementYoutubeComment {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i64>,
    /// Display name of the comment author.
    pub author_name: Option<String>,
    /// URL of the author's avatar image.
    pub author_thumbnail: Option<String>,
    /// URL of the author's channel.
    pub author_url: Option<String>,
    /// Text content.
    pub text: Option<String>,
    /// Human-readable publication date.
    pub publication_date: Option<String>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// Number of likes.
    pub likes_count: Option<i64>,
    /// Number of replies to the comment.
    pub reply_count: Option<i64>,
}
