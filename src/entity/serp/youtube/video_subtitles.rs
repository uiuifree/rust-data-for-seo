use serde::{Deserialize, Serialize};

/// YouTube video-subtitles advanced result object.
/// See <https://docs.dataforseo.com/v3/serp/youtube/video_subtitles/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiYoutubeVideoSubtitlesTaskAdvanced {
    /// YouTube video identifier.
    pub video_id: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Language code of the original subtitles.
    pub origin_language: Option<String>,
    /// Language code the subtitles were translated to.
    pub translate_language: Option<String>,
    /// `true` if the requested subtitle language is unsupported.
    pub unsupported_language: Option<bool>,
    /// Number of subtitle lines returned.
    pub subtitles_count: Option<i64>,
    /// Number of items returned in this result.
    pub items_count: Option<i64>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiElementYoutubeSubtitles>>,
}

/// `youtube_subtitles` item: one caption line with its timing.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementYoutubeSubtitles {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i64>,
    /// Text content.
    pub text: Option<String>,
    /// Start time in seconds.
    pub start_time: Option<f64>,
    /// End time in seconds.
    pub end_time: Option<f64>,
    /// Duration as a formatted string.
    pub duration_time: Option<f64>,
}
