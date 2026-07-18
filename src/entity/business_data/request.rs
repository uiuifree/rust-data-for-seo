//! Request bodies shared by Business Data `task_post` / `live` endpoints.

use serde::{Deserialize, Serialize};

/// Task request shared by the simple Google Business Data `task_post` endpoints
/// (My Business Info, My Business Updates, Questions & Answers).
/// See <https://docs.dataforseo.com/v3/business_data/google/my_business_info/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiGoogleTaskPost {
    /// Business name, or a `cid:` / `place_id:` identifier to look up.
    pub keyword: String,
    /// Full location name, e.g. `"London,England,United Kingdom"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code from the `locations` endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// GPS coordinates as `"latitude,longitude"` or `"latitude,longitude,radius"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name, e.g. `"English"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code, e.g. `"en"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Task priority: `1` (normal, default) or `2` (high, extra cost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL that receives the finished results via POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// URL notified via GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl BusinessDataApiGoogleTaskPost {
    /// Builds a task request for the given business name, `cid:`, or `place_id:`.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        BusinessDataApiGoogleTaskPost {
            keyword: keyword.into(),
            ..BusinessDataApiGoogleTaskPost::default()
        }
    }
}

/// Task request for the Google Reviews and Extended Reviews `task_post` endpoints.
/// See <https://docs.dataforseo.com/v3/business_data/google/reviews/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiReviewsTaskPost {
    /// Business name, or a `cid:` / `place_id:` identifier to look up.
    pub keyword: String,
    /// Full location name, e.g. `"London,England,United Kingdom"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code from the `locations` endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// GPS coordinates as `"latitude,longitude"` or `"latitude,longitude,radius"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name, e.g. `"English"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code, e.g. `"en"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Number of reviews to parse (default `10`, in multiples of 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Review ordering, e.g. `"relevant"`, `"newest"`, `"highest_rating"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Task priority: `1` (normal, default) or `2` (high, extra cost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL that receives the finished results via POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// URL notified via GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl BusinessDataApiReviewsTaskPost {
    /// Builds a reviews task request for the given business name, `cid:`, or `place_id:`.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        BusinessDataApiReviewsTaskPost {
            keyword: keyword.into(),
            ..BusinessDataApiReviewsTaskPost::default()
        }
    }
}

/// Task request for the Google Hotel Searches `task_post` / `live` endpoints.
/// See <https://docs.dataforseo.com/v3/business_data/google/hotel_searches/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiHotelSearchesTaskPost {
    /// Search query, typically a destination or hotel name.
    pub keyword: String,
    /// Full location name, e.g. `"London,England,United Kingdom"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code from the `locations` endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// GPS coordinates as `"latitude,longitude"` or `"latitude,longitude,radius"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name, e.g. `"English"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code, e.g. `"en"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Check-in date in `yyyy-mm-dd` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in: Option<String>,
    /// Check-out date in `yyyy-mm-dd` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_out: Option<String>,
    /// Three-letter currency code prices are returned in (e.g. `"USD"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Number of adult guests to price the stay for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adults: Option<i32>,
    /// Number of child guests to price the stay for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<i32>,
    /// Task priority: `1` (normal, default) or `2` (high, extra cost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL that receives the finished results via POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// URL notified via GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl BusinessDataApiHotelSearchesTaskPost {
    /// Builds a hotel searches task request for the given keyword.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        BusinessDataApiHotelSearchesTaskPost {
            keyword: keyword.into(),
            ..BusinessDataApiHotelSearchesTaskPost::default()
        }
    }
}

/// Task request for the Google Hotel Info `task_post` / `live` endpoints.
/// See <https://docs.dataforseo.com/v3/business_data/google/hotel_info/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiHotelInfoTaskPost {
    /// Hotel name or `hotel_identifier:` value identifying the property.
    pub keyword: String,
    /// Full location name, e.g. `"London,England,United Kingdom"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code from the `locations` endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i64>,
    /// Full language name, e.g. `"English"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code, e.g. `"en"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Check-in date in `yyyy-mm-dd` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in: Option<String>,
    /// Check-out date in `yyyy-mm-dd` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_out: Option<String>,
    /// Three-letter currency code prices are returned in (e.g. `"USD"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Task priority: `1` (normal, default) or `2` (high, extra cost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL that receives the finished results via POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// URL notified via GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl BusinessDataApiHotelInfoTaskPost {
    /// Builds a hotel info task request for the given keyword.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        BusinessDataApiHotelInfoTaskPost {
            keyword: keyword.into(),
            ..BusinessDataApiHotelInfoTaskPost::default()
        }
    }
}
