use serde::{Deserialize, Serialize};
use serde_json::Value;

/// YouTube video-info advanced result object.
/// See <https://docs.dataforseo.com/v3/serp/youtube/video_info/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiYoutubeVideoInfoTaskAdvanced {
    /// YouTube video identifier.
    pub video_id: Option<String>,
    /// Search-engine domain the results were taken from.
    pub se_domain: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: Option<i32>,
    /// Language code the search was run for.
    pub language_code: Option<String>,
    /// Direct URL to reproduce the search on the search engine.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was received.
    pub datetime: Option<String>,
    /// Search-engine spelling correction applied to the query, if any.
    pub spell: Option<Value>,
    /// Distinct element types present in the returned SERP.
    pub item_types: Option<Vec<String>>,
    /// Number of items returned in this result.
    pub items_count: Option<i64>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiElementYoutubeVideoInfo>>,
}

/// `youtube_video_info` item describing a single video.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementYoutubeVideoInfo {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// YouTube video identifier.
    pub video_id: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// URL of the thumbnail image.
    pub thumbnail_url: Option<String>,
    /// Category of the entity.
    pub category: Option<String>,
    /// YouTube channel identifier.
    pub channel_id: Option<String>,
    /// Name of the YouTube channel.
    pub channel_name: Option<String>,
    /// URL of the YouTube channel.
    pub channel_url: Option<String>,
    /// URL of the channel's logo image.
    pub channel_logo: Option<String>,
    /// Subscriber count of the channel.
    pub channel_subscribers_count: Option<SerpApiYoutubeSubscribersCount>,
    /// Number of views.
    pub views_count: Option<i64>,
    /// Number of likes.
    pub likes_count: Option<i64>,
    /// Total number of comments on the video.
    pub comments_count: Option<i64>,
    /// Human-readable publication date.
    pub publication_date: Option<String>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// `true` if the video is a live stream.
    pub is_live: Option<bool>,
    /// `true` if the video can be embedded.
    pub is_embeddable: Option<bool>,
    /// Duration as a formatted string.
    pub duration_time: Option<String>,
    /// Duration in seconds.
    pub duration_time_seconds: Option<i32>,
    /// Keywords.
    pub keywords: Option<Vec<String>>,
    /// Subtitle tracks available for the video.
    pub subtitles: Option<Vec<SerpApiYoutubeVideoSubtitleTrack>>,
    /// Streaming-quality options available for the video.
    pub streaming_quality: Option<Vec<SerpApiYoutubeStreamingQuality>>,
}

/// Channel subscriber count with both the raw and displayed forms.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiYoutubeSubscribersCount {
    /// Count as displayed (e.g. "1.2M").
    pub displayed_count: Option<String>,
    /// Numeric count.
    pub count: Option<i64>,
}

/// A subtitle track available for the video.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiYoutubeVideoSubtitleTrack {
    /// Language code of the track.
    pub language: Option<String>,
    /// `true` if the subtitle track can be auto-translated.
    pub is_translatable: Option<bool>,
    /// `true` if the subtitles were auto-generated.
    pub is_auto_generated: Option<bool>,
}

/// A streaming-quality option available for the video.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiYoutubeStreamingQuality {
    /// Human-readable label.
    pub label: Option<String>,
    /// Width of the bounding box, in pixels.
    pub width: Option<i32>,
    /// Height of the bounding box, in pixels.
    pub height: Option<i32>,
    /// Bitrate of the stream, in bits per second.
    pub bitrate: Option<i64>,
    /// MIME type of the stream.
    pub mime_type: Option<String>,
    /// Frames per second of the stream.
    pub fps: Option<i32>,
}
