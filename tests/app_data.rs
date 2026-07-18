//! Tests for the App Data API domain.
//!
//! The deserialization tests use JSON literals shaped after the DataForSEO
//! documentation samples and require no credentials. Live tests hit the real
//! API and are ignored by default.

use data_for_seo::entity::{
    AppDataApiAppInfoResult, AppDataApiAppReviewsResult, AppDataApiAppSearchesResult,
    DataForSeoApiResponseData,
};

#[test]
fn deserialize_app_searches_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.002,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012345",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.002,
            "result_count": 1,
            "path": ["v3", "app_data", "google", "app_searches", "task_get", "advanced"],
            "data": {"api": "app_data", "keyword": "weather"},
            "result": [{
                "keyword": "weather",
                "type": "app_searches",
                "se_domain": "play.google.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://play.google.com/store/search?q=weather",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "item_types": ["google_play_search_organic"],
                "se_results_count": 250,
                "items_count": 1,
                "items": [{
                    "type": "google_play_search_organic",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "position": "left",
                    "app_id": "com.example.weather",
                    "title": "Weather App",
                    "url": "https://play.google.com/store/apps/details?id=com.example.weather",
                    "icon": "https://example.com/icon.png",
                    "description": "Best weather app",
                    "reviews_count": 12345,
                    "rating": {
                        "rating_type": "Max5",
                        "value": 4.5,
                        "votes_count": 12345,
                        "rating_max": 5
                    },
                    "is_free": true,
                    "price": {
                        "current": 0.0,
                        "regular": null,
                        "max_value": null,
                        "currency": "USD",
                        "is_price_range": false,
                        "displayed_price": null
                    },
                    "developer": "Example Inc.",
                    "developer_url": "https://play.google.com/store/apps/dev?id=123"
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<AppDataApiAppSearchesResult> =
        serde_json::from_str(json).expect("app_searches response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.keyword.as_deref(), Some("weather"));
    assert_eq!(result.items_count, Some(1));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.app_id.as_deref(), Some("com.example.weather"));
    assert_eq!(item.rating.as_ref().unwrap().value, Some(4.5));
    assert_eq!(item.is_free, Some(true));
    assert_eq!(
        item.price.as_ref().unwrap().currency.as_deref(),
        Some("USD")
    );
}

#[test]
fn deserialize_app_info_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.002,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012346",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.002,
            "path": ["v3", "app_data", "google", "app_info", "task_get", "advanced"],
            "data": {"api": "app_data", "app_id": "com.example.weather"},
            "result": [{
                "app_id": "com.example.weather",
                "type": "app_info",
                "se_domain": "play.google.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://play.google.com/store/apps/details?id=com.example.weather",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "items_count": 1,
                "items": [{
                    "type": "google_play_info_organic",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "position": "left",
                    "app_id": "com.example.weather",
                    "title": "Weather App",
                    "url": "https://play.google.com/store/apps/details?id=com.example.weather",
                    "icon": "https://example.com/icon.png",
                    "description": "Best weather app",
                    "reviews_count": 12345,
                    "rating": {
                        "rating_type": "Max5",
                        "value": 4.5,
                        "votes_count": 12345,
                        "rating_max": 5
                    },
                    "is_free": true,
                    "price": {"current": 0.0, "currency": "USD", "is_price_range": false},
                    "main_category": "Weather",
                    "installs": "1,000,000+",
                    "installs_count": 1000000,
                    "developer": "Example Inc.",
                    "developer_id": "123",
                    "developer_url": "https://play.google.com/store/apps/dev?id=123",
                    "developer_email": "dev@example.com",
                    "version": "2.3.1",
                    "minimum_os_version": "5.0",
                    "size": "24M",
                    "released_date": "2020-01-01",
                    "last_update_date": "2024-01-01",
                    "images": ["https://example.com/1.png", "https://example.com/2.png"],
                    "genres": ["Weather"],
                    "tags": ["weather", "forecast"]
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<AppDataApiAppInfoResult> =
        serde_json::from_str(json).expect("app_info response should deserialize");

    let result = response.first_result().expect("one result expected");
    assert_eq!(result.app_id.as_deref(), Some("com.example.weather"));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.installs_count, Some(1_000_000));
    assert_eq!(item.version.as_deref(), Some("2.3.1"));
    assert_eq!(item.images.as_ref().unwrap().len(), 2);
    assert_eq!(item.main_category.as_deref(), Some("Weather"));
}

#[test]
fn deserialize_app_reviews_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.002,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012347",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.002,
            "path": ["v3", "app_data", "google", "app_reviews", "task_get", "advanced"],
            "data": {"api": "app_data", "app_id": "com.example.weather"},
            "result": [{
                "app_id": "com.example.weather",
                "type": "app_reviews",
                "se_domain": "play.google.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://play.google.com/store/apps/details?id=com.example.weather",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "title": "Weather App",
                "rating": {"rating_type": "Max5", "value": 4.5, "votes_count": 12345, "rating_max": 5},
                "reviews_count": 12345,
                "items_count": 1,
                "items": [{
                    "type": "google_play_reviews_search",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "position": "left",
                    "id": "review-abc-123",
                    "version": "2.3.1",
                    "title": null,
                    "review_text": "Great app, very accurate!",
                    "timestamp": "2023-12-31 10:00:00 +00:00",
                    "helpful_count": 42,
                    "rating": {"rating_type": "Max5", "value": 5.0, "votes_count": null, "rating_max": 5},
                    "user_profile": {
                        "profile_name": "Jane Doe",
                        "profile_image_url": "https://example.com/jane.png"
                    },
                    "responses": [{
                        "author": "Example Inc.",
                        "title": null,
                        "text": "Thanks for your feedback!",
                        "timestamp": "2024-01-01 09:00:00 +00:00"
                    }]
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<AppDataApiAppReviewsResult> =
        serde_json::from_str(json).expect("app_reviews response should deserialize");

    let result = response.first_result().expect("one result expected");
    assert_eq!(result.reviews_count, Some(12345));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.id.as_deref(), Some("review-abc-123"));
    assert_eq!(
        item.review_text.as_deref(),
        Some("Great app, very accurate!")
    );
    assert_eq!(item.helpful_count, Some(42));
    assert_eq!(
        item.user_profile.as_ref().unwrap().profile_name.as_deref(),
        Some("Jane Doe")
    );
    assert_eq!(item.responses.as_ref().unwrap().len(), 1);
}

#[cfg(test)]
mod live {
    use data_for_seo::{
        AppDataApiAppInfoTaskPostRequest, AppDataApiAppSearchesTaskPostRequest, DataForSeoClient,
    };
    use std::env;

    fn client() -> DataForSeoClient {
        dotenv::dotenv().ok();
        let id = env::var("ID").unwrap();
        let pass = env::var("PASSWORD").unwrap();
        DataForSeoClient::new(id, pass)
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn google_locations() {
        let client = client();
        let response = client.app_data().google().locations().await.unwrap();
        assert_eq!(response.status_code, 20000);
        assert_ne!(response.results().count(), 0);
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn google_app_searches_task_post() {
        let client = client();
        let request = AppDataApiAppSearchesTaskPostRequest::new(
            "weather".to_string(),
            2840,
            "en".to_string(),
        );
        let response = client
            .app_data()
            .google()
            .app_searches_task_post(vec![request])
            .await
            .unwrap();
        assert_eq!(response.status_code, 20000);
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn google_app_info_task_post() {
        let client = client();
        let request = AppDataApiAppInfoTaskPostRequest::new(
            "com.rovio.angrybirds".to_string(),
            2840,
            "en".to_string(),
        );
        let response = client
            .app_data()
            .google()
            .app_info_task_post(vec![request])
            .await
            .unwrap();
        assert_eq!(response.status_code, 20000);
    }
}
