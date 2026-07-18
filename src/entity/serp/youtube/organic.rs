use crate::entity::SerpApiSearchResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// YouTube organic advanced result.
/// See <https://docs.dataforseo.com/v3/serp/youtube/organic/task_get/advanced/>.
pub type SerpApiYoutubeOrganicTaskAdvanced = SerpApiSearchResult<SerpApiYoutubeOrganicItem>;

/// A single item in a YouTube organic SERP, tagged by the DataForSEO `type` field.
/// Unrecognized `type` values fall back to [`SerpApiYoutubeOrganicItem::Unknown`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SerpApiYoutubeOrganicItem {
    /// Element of type `youtube_video`.
    #[serde(rename = "youtube_video")]
    YoutubeVideo(Box<SerpApiElementYoutubeVideo>),
    /// Element of type `youtube_video_paid`.
    #[serde(rename = "youtube_video_paid")]
    YoutubeVideoPaid(Box<SerpApiElementYoutubeVideo>),
    /// Element of type `youtube_channel`.
    #[serde(rename = "youtube_channel")]
    YoutubeChannel(Box<SerpApiElementYoutubeChannel>),
    /// Element of type `youtube_playlist`.
    #[serde(rename = "youtube_playlist")]
    YoutubePlaylist(Box<SerpApiElementYoutubePlaylist>),
    /// Fallback holding the raw JSON of an unrecognized `type`.
    #[serde(untagged)]
    Unknown(Value),
}

/// `youtube_video` / `youtube_video_paid` organic item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementYoutubeVideo {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i64>,
    /// Rank of the element among result blocks.
    pub block_rank: Option<i64>,
    /// Name of the result block the element belongs to.
    pub block_name: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// YouTube video identifier.
    pub video_id: Option<String>,
    /// URL of the thumbnail image.
    pub thumbnail_url: Option<String>,
    /// YouTube channel identifier.
    pub channel_id: Option<String>,
    /// Name of the YouTube channel.
    pub channel_name: Option<String>,
    /// URL of the YouTube channel.
    pub channel_url: Option<String>,
    /// URL of the channel's logo image.
    pub channel_logo: Option<String>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// Substrings the search engine highlighted in the result.
    pub highlighted: Option<Vec<String>>,
    /// Badges shown on the result.
    pub badges: Option<Vec<String>>,
    /// `true` if the video is a live stream.
    pub is_live: Option<bool>,
    /// `true` if the video is a YouTube Short.
    pub is_shorts: Option<bool>,
    /// `true` if the entry is a movie.
    pub is_movie: Option<bool>,
    /// Number of views.
    pub views_count: Option<i64>,
    /// Human-readable publication date.
    pub publication_date: Option<String>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// Duration as a formatted string.
    pub duration_time: Option<String>,
    /// Duration in seconds.
    pub duration_time_seconds: Option<i64>,
}

/// `youtube_channel` organic item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementYoutubeChannel {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i64>,
    /// Rank of the element among result blocks.
    pub block_rank: Option<i64>,
    /// Name of the result block the element belongs to.
    pub block_name: Option<String>,
    /// YouTube channel identifier.
    pub channel_id: Option<String>,
    /// Name of the entity.
    pub name: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Logo.
    pub logo: Option<String>,
    /// Number of videos.
    pub video_count: Option<i64>,
    /// `true` if the channel is verified.
    pub is_verified: Option<bool>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// Substrings the search engine highlighted in the result.
    pub highlighted: Option<Vec<String>>,
}

/// `youtube_playlist` organic item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementYoutubePlaylist {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i64>,
    /// Rank of the element among result blocks.
    pub block_rank: Option<i64>,
    /// Name of the result block the element belongs to.
    pub block_name: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// YouTube playlist identifier.
    pub playlist_id: Option<String>,
    /// URL of the thumbnail image.
    pub thumbnail_url: Option<String>,
    /// YouTube channel identifier.
    pub channel_id: Option<String>,
    /// Name of the YouTube channel.
    pub channel_name: Option<String>,
    /// URL of the YouTube channel.
    pub channel_url: Option<String>,
    /// URL of the channel's logo image.
    pub channel_logo: Option<String>,
    /// Number of videos.
    pub videos_count: Option<i64>,
    /// Preview videos shown for the playlist.
    pub preview_videos: Option<Vec<SerpApiElementYoutubePreviewVideo>>,
}

/// A preview video nested inside a `youtube_playlist` item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementYoutubePreviewVideo {
    /// YouTube video identifier.
    pub video_id: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Duration as a formatted string.
    pub duration_time: Option<String>,
    /// Duration in seconds.
    pub duration_time_second: Option<i64>,
}
