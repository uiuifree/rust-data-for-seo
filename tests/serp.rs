//! SERP API tests.
//!
//! Live tests hit the DataForSEO API and require an `.env` file with `ID` and
//! `PASSWORD`; they are marked `#[ignore]`. The deserialization tests at the
//! bottom run offline and validate the response models against sample payloads.

use data_for_seo::DataForSeoClient;
use data_for_seo::entity::{
    DataForSeoApiResponseData, SerpApiGoogleFinanceTickerSearchItem,
    SerpApiGoogleFinanceTickerSearchTaskAdvanced, SerpApiGoogleMapsItem,
    SerpApiGoogleMapsTaskAdvanced, SerpApiGoogleOrganicItem, SerpApiGoogleOrganicTaskAdvanced,
    SerpApiYoutubeOrganicItem, SerpApiYoutubeOrganicTaskAdvanced,
};
use data_for_seo::{SerpApiGoogleOrganicTaskPostRequest, SerpApiNaverOrganicTaskPostRequest};
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();
    DataForSeoClient::new(id, pass)
}

// ---------------------------------------------------------------------------
// Live integration tests (require credentials)
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn locations() {
    let client = client();
    let response = client.serp().google().locations().await.unwrap();
    assert!(
        response
            .results()
            .any(|l| l.location_name.as_deref() == Some("Japan"))
    );
}

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn languages() {
    let client = client();
    let response = client.serp().google().languages().await.unwrap();
    assert_ne!(response.results().count(), 0);
}

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn task_ready() {
    let client = client();
    let response = client.serp().task_ready().await.unwrap();
    for task in response.tasks() {
        println!("{}", task.id);
        for result in task.results() {
            println!("{result:?}");
        }
    }
}

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn organic_task_post_then_get() {
    let client = client();
    let mut request = SerpApiGoogleOrganicTaskPostRequest::new("ja".to_string(), 1009307);
    request.keyword = "介護".to_string();
    let posted = client
        .serp()
        .google()
        .organic_task_post(vec![request])
        .await
        .unwrap();
    for task in posted.tasks() {
        println!("posted task {}", task.id);
    }
}

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn organic_live_advanced() {
    let client = client();
    let mut request = SerpApiGoogleOrganicTaskPostRequest::new("ja".to_string(), 1009307);
    request.keyword = "介護".to_string();
    let response = client
        .serp()
        .google()
        .organic_live_advanced(vec![request])
        .await
        .unwrap();
    for result in response.results() {
        for item in result.items.iter().flatten() {
            if let SerpApiGoogleOrganicItem::Organic(organic) = item {
                println!("{:?} {:?}", organic.rank_absolute, organic.url);
            }
        }
    }
}

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn naver_organic_task_post() {
    let client = client();
    let request = SerpApiNaverOrganicTaskPostRequest::new("맛집");
    let posted = client
        .serp()
        .naver()
        .organic_task_post(vec![request])
        .await
        .unwrap();
    for task in posted.tasks() {
        println!("posted naver task {}", task.id);
    }
}

// ---------------------------------------------------------------------------
// Offline deserialization tests (no credentials required)
// ---------------------------------------------------------------------------

#[test]
fn deserialize_google_organic_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.1 sec",
        "cost": 0.002,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011543-1535-0066-0000-17d3dd1a1b2c",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec",
            "cost": 0.002,
            "result_count": 1,
            "path": ["v3", "serp", "google", "organic", "task_get", "advanced"],
            "data": {"api": "serp"},
            "result": [{
                "keyword": "seo",
                "type": "organic",
                "se_domain": "google.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://www.google.com/search?q=seo",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "spell": null,
                "item_types": ["organic", "top_stories"],
                "se_results_count": 123456,
                "items_count": 2,
                "items": [
                    {
                        "type": "organic",
                        "rank_group": 1,
                        "rank_absolute": 1,
                        "domain": "example.com",
                        "title": "Example",
                        "url": "https://example.com/",
                        "description": "An example result"
                    },
                    {
                        "type": "top_stories",
                        "rank_group": 1,
                        "rank_absolute": 2,
                        "title": "Breaking"
                    }
                ]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<SerpApiGoogleOrganicTaskAdvanced> =
        serde_json::from_str(json).expect("organic response should deserialize");

    let result = response.first_result().expect("one result");
    assert_eq!(result.keyword, "seo");
    assert_eq!(result.items_count, Some(2));

    let items = result.items.as_ref().expect("items present");
    match &items[0] {
        SerpApiGoogleOrganicItem::Organic(organic) => {
            assert_eq!(organic.domain.as_deref(), Some("example.com"));
            assert_eq!(organic.rank_absolute, Some(1));
        }
        other => panic!("expected organic item, got {other:?}"),
    }
    // `top_stories` is not a modeled variant, so it must land in `Unknown`.
    assert!(matches!(items[1], SerpApiGoogleOrganicItem::Unknown(_)));
}

#[test]
fn deserialize_youtube_organic_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.1 sec",
        "cost": 0.002,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011543-1535-0066-0000-17d3dd1a1b2d",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec",
            "cost": 0.002,
            "path": ["v3", "serp", "youtube", "organic", "task_get", "advanced"],
            "data": {"api": "serp"},
            "result": [{
                "keyword": "rust programming",
                "type": "youtube_organic",
                "se_domain": "youtube.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://www.youtube.com/results?search_query=rust+programming",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "item_types": ["youtube_video", "youtube_channel"],
                "items_count": 2,
                "items": [
                    {
                        "type": "youtube_video",
                        "rank_group": 1,
                        "rank_absolute": 1,
                        "title": "Rust in 100 seconds",
                        "video_id": "abc123",
                        "channel_name": "Fireship",
                        "views_count": 1000000
                    },
                    {
                        "type": "youtube_channel",
                        "rank_group": 1,
                        "rank_absolute": 2,
                        "name": "The Rust Programming Language",
                        "is_verified": true
                    }
                ]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<SerpApiYoutubeOrganicTaskAdvanced> =
        serde_json::from_str(json).expect("youtube response should deserialize");

    let result = response.first_result().expect("one result");
    assert_eq!(result.keyword.as_deref(), Some("rust programming"));

    let items = result.items.as_ref().expect("items present");
    match &items[0] {
        SerpApiYoutubeOrganicItem::YoutubeVideo(video) => {
            assert_eq!(video.video_id.as_deref(), Some("abc123"));
            assert_eq!(video.views_count, Some(1_000_000));
        }
        other => panic!("expected youtube_video, got {other:?}"),
    }
    match &items[1] {
        SerpApiYoutubeOrganicItem::YoutubeChannel(channel) => {
            assert_eq!(channel.is_verified, Some(true));
        }
        other => panic!("expected youtube_channel, got {other:?}"),
    }
}

#[test]
fn deserialize_google_maps_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.1 sec",
        "cost": 0.002,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011543-1535-0066-0000-17d3dd1a1b2e",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec",
            "cost": 0.002,
            "path": ["v3", "serp", "google", "maps", "task_get", "advanced"],
            "data": {"api": "serp"},
            "result": [{
                "keyword": "coffee",
                "type": "maps",
                "se_domain": "google.com",
                "location_code": 1023191,
                "language_code": "en",
                "items_count": 1,
                "items": [
                    {
                        "type": "maps_search",
                        "rank_group": 1,
                        "rank_absolute": 1,
                        "title": "Blue Bottle Coffee",
                        "place_id": "ChIJ123",
                        "cid": "987654321",
                        "latitude": 35.6812,
                        "longitude": 139.7671,
                        "rating": {"rating_type": "Max5", "value": 4.5, "votes_count": 1200, "rating_max": 5}
                    }
                ]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<SerpApiGoogleMapsTaskAdvanced> =
        serde_json::from_str(json).expect("maps response should deserialize");

    let result = response.first_result().expect("one result");
    let items = result.items.as_ref().expect("items present");
    match &items[0] {
        SerpApiGoogleMapsItem::MapsSearch(place) => {
            assert_eq!(place.title.as_deref(), Some("Blue Bottle Coffee"));
            assert_eq!(place.place_id.as_deref(), Some("ChIJ123"));
            assert_eq!(place.latitude, Some(35.6812));
        }
        other => panic!("expected maps_search, got {other:?}"),
    }
}

#[test]
fn deserialize_google_finance_ticker_search_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.1 sec",
        "cost": 0.002,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011543-1535-0066-0000-17d3dd1a1c30",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec",
            "cost": 0.002,
            "path": ["v3", "serp", "google", "finance_ticker_search", "task_get", "advanced"],
            "data": {"api": "serp"},
            "result": [{
                "keyword": "apple",
                "type": "finance_ticker_search",
                "se_domain": "google.com",
                "location_code": 2840,
                "language_code": "en",
                "item_types": ["google_finance_market_instrument", "google_finance_market_index"],
                "items_count": 2,
                "items": [
                    {
                        "type": "google_finance_market_instrument",
                        "rank_group": 1,
                        "rank_absolute": 1,
                        "ticker": "AAPL",
                        "price": 189.5,
                        "price_delta": 1.25,
                        "price_currency": "USD",
                        "displayed_name": "Apple Inc",
                        "trend": "up",
                        "percentage_delta": 0.66
                    },
                    {
                        "type": "google_finance_market_index",
                        "rank_group": 2,
                        "rank_absolute": 2,
                        "ticker": ".INX",
                        "market_identifier": "INDEXSP",
                        "index_value": 4500.5,
                        "displayed_name": "S&P 500"
                    }
                ]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<SerpApiGoogleFinanceTickerSearchTaskAdvanced> =
        serde_json::from_str(json).expect("finance ticker search should deserialize");

    let result = response.first_result().expect("one result");
    let items = result.items.as_ref().expect("items present");
    match &items[0] {
        SerpApiGoogleFinanceTickerSearchItem::MarketInstrument(instrument) => {
            assert_eq!(instrument.ticker.as_deref(), Some("AAPL"));
            assert_eq!(instrument.price, Some(189.5));
            assert_eq!(instrument.price_currency.as_deref(), Some("USD"));
        }
        other => panic!("expected market_instrument, got {other:?}"),
    }
    match &items[1] {
        SerpApiGoogleFinanceTickerSearchItem::MarketIndex(index) => {
            assert_eq!(index.market_identifier.as_deref(), Some("INDEXSP"));
            assert_eq!(index.index_value, Some(4500.5));
        }
        other => panic!("expected market_index, got {other:?}"),
    }
}
