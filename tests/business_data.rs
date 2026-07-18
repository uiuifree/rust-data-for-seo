//! Tests for the Business Data API domain.
//!
//! The deserialization tests use JSON literals shaped after the DataForSEO
//! documentation samples and require no credentials. Live tests hit the real
//! API and are ignored by default.

use data_for_seo::entity::{
    BusinessDataApiBusinessListingsSearchResult, BusinessDataApiMyBusinessInfoResult,
    BusinessDataApiPinterestItem, BusinessDataApiQuestionsAndAnswersResult,
    BusinessDataApiReviewsResult, DataForSeoApiResponseData,
};

#[test]
fn deserialize_my_business_info() {
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
            "path": ["v3", "business_data", "google", "my_business_info", "task_get"],
            "data": {"api": "business_data", "keyword": "Starbucks"},
            "result": [{
                "keyword": "Starbucks",
                "type": "my_business_info",
                "se_domain": "google.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://www.google.com/search?q=Starbucks",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "item_types": ["business_info"],
                "items_count": 1,
                "items": [{
                    "type": "business_info",
                    "title": "Starbucks",
                    "description": "Coffeehouse chain",
                    "category": "Coffee shop",
                    "category_ids": ["coffee_shop"],
                    "cid": "123456789",
                    "feature_id": "0x0:0xabc",
                    "address": "123 Main St, Seattle, WA 98101",
                    "address_info": {
                        "borough": null,
                        "address": "123 Main St",
                        "city": "Seattle",
                        "zip": "98101",
                        "region": "Washington",
                        "country_code": "US"
                    },
                    "place_id": "ChIJabc",
                    "phone": "+12065551234",
                    "url": "https://www.starbucks.com",
                    "domain": "starbucks.com",
                    "total_photos": 42,
                    "latitude": 47.6062,
                    "longitude": -122.3321,
                    "is_claimed": true,
                    "rating": {
                        "rating_type": "Max5",
                        "value": 4.3,
                        "votes_count": 512,
                        "rating_max": 5
                    },
                    "rating_distribution": {"1": 10, "2": 5, "3": 30, "4": 120, "5": 347},
                    "work_time": {"work_hours": {}, "current_status": "open"},
                    "price_level": "$$"
                }]
            }]
        }]
    }"#;

    let parsed: DataForSeoApiResponseData<BusinessDataApiMyBusinessInfoResult> =
        serde_json::from_str(json).expect("my_business_info deserializes");
    let result = parsed.first_result().expect("has a result");
    assert_eq!(result.keyword.as_deref(), Some("Starbucks"));
    let item = &result.items()[0];
    assert_eq!(item.title.as_deref(), Some("Starbucks"));
    assert_eq!(item.rating.as_ref().and_then(|r| r.value), Some(4.3));
    assert_eq!(
        item.address_info.as_ref().and_then(|a| a.city.as_deref()),
        Some("Seattle")
    );
    assert_eq!(item.is_claimed, Some(true));
}

#[test]
fn deserialize_reviews() {
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
            "result_count": 1,
            "path": ["v3", "business_data", "google", "reviews", "task_get"],
            "data": {"api": "business_data", "keyword": "Starbucks"},
            "result": [{
                "keyword": "Starbucks",
                "type": "reviews",
                "se_domain": "google.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://www.google.com/search?q=Starbucks",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "title": "Starbucks",
                "feature_id": "0x0:0xabc",
                "cid": "123456789",
                "reviews_count": 512,
                "rating": {"rating_type": "Max5", "value": 4.3, "votes_count": 512, "rating_max": 5},
                "items_count": 1,
                "items": [{
                    "type": "google_review",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "position": "left",
                    "review_text": "Great coffee and friendly staff.",
                    "time_ago": "a week ago",
                    "timestamp": "2023-12-25 08:00:00 +00:00",
                    "rating": {"rating_type": "Max5", "value": 5, "votes_count": null, "rating_max": 5},
                    "local_guide": true,
                    "profile_name": "Jane Doe",
                    "profile_url": "https://www.google.com/maps/contrib/1",
                    "owner_answer": "Thank you!",
                    "review_id": "review-1"
                }]
            }]
        }]
    }"#;

    let parsed: DataForSeoApiResponseData<BusinessDataApiReviewsResult> =
        serde_json::from_str(json).expect("reviews deserializes");
    let result = parsed.first_result().expect("has a result");
    assert_eq!(result.reviews_count, Some(512));
    let item = &result.items()[0];
    assert_eq!(
        item.review_text.as_deref(),
        Some("Great coffee and friendly staff.")
    );
    assert_eq!(item.local_guide, Some(true));
    assert_eq!(item.profile_name.as_deref(), Some("Jane Doe"));
}

#[test]
fn deserialize_business_listings_search() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.02,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012347",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.02,
            "result_count": 1,
            "path": ["v3", "business_data", "business_listings", "search", "live"],
            "data": {"api": "business_data", "function": "business_listings"},
            "result": [{
                "total_count": 2500,
                "count": 1,
                "offset": 0,
                "offset_token": "token-abc",
                "items_count": 1,
                "items": [{
                    "type": "business_listing",
                    "title": "Blue Bottle Coffee",
                    "category": "Coffee shop",
                    "cid": "987654321",
                    "address": "456 Market St, San Francisco, CA",
                    "address_info": {"city": "San Francisco", "country_code": "US"},
                    "latitude": 37.7749,
                    "longitude": -122.4194,
                    "is_claimed": false,
                    "total_photos": 15,
                    "rating": {"rating_type": "Max5", "value": 4.6, "votes_count": 800, "rating_max": 5}
                }]
            }]
        }]
    }"#;

    let parsed: DataForSeoApiResponseData<BusinessDataApiBusinessListingsSearchResult> =
        serde_json::from_str(json).expect("business listings search deserializes");
    let result = parsed.first_result().expect("has a result");
    assert_eq!(result.total_count, Some(2500));
    assert_eq!(result.offset_token.as_deref(), Some("token-abc"));
    let item = &result.items()[0];
    assert_eq!(item.title.as_deref(), Some("Blue Bottle Coffee"));
    assert_eq!(item.is_claimed, Some(false));
    assert_eq!(item.rating.as_ref().and_then(|r| r.votes_count), Some(800));
}

#[test]
fn deserialize_questions_and_answers() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.002,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012348",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.002,
            "result_count": 1,
            "path": ["v3", "business_data", "google", "questions_and_answers", "task_get"],
            "data": {"api": "business_data", "keyword": "Starbucks"},
            "result": [{
                "keyword": "Starbucks",
                "type": "questions_and_answers",
                "se_domain": "google.com",
                "location_code": 2840,
                "language_code": "en",
                "cid": "123456789",
                "items_count": 1,
                "items": [{
                    "type": "google_business_question_item",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "question_id": "q-1",
                    "profile_name": "Curious Customer",
                    "question_text": "Do you have oat milk?",
                    "time_ago": "2 weeks ago",
                    "timestamp": "2023-12-18 10:00:00 +00:00",
                    "items": [{
                        "type": "google_business_answer_element",
                        "answer_id": "a-1",
                        "profile_name": "Starbucks",
                        "answer_text": "Yes, we do!",
                        "time_ago": "2 weeks ago",
                        "timestamp": "2023-12-18 12:00:00 +00:00",
                        "votes_count": 3
                    }]
                }]
            }]
        }]
    }"#;

    let parsed: DataForSeoApiResponseData<BusinessDataApiQuestionsAndAnswersResult> =
        serde_json::from_str(json).expect("questions and answers deserializes");
    let result = parsed.first_result().expect("has a result");
    let question = &result.items()[0];
    assert_eq!(
        question.question_text.as_deref(),
        Some("Do you have oat milk?")
    );
    let answers = question.items.as_deref().unwrap_or_default();
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0].answer_text.as_deref(), Some("Yes, we do!"));
    assert_eq!(answers[0].votes_count, Some(3));
}

#[test]
fn deserialize_pinterest_live() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.001,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012349",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.001,
            "result_count": 1,
            "path": ["v3", "business_data", "social_media", "pinterest", "live"],
            "data": {"api": "business_data", "function": "pinterest"},
            "result": [{
                "type": "social_media_pinterest_item",
                "page_url": "https://www.example.com/recipe",
                "pins_count": 1280
            }]
        }]
    }"#;

    let parsed: DataForSeoApiResponseData<BusinessDataApiPinterestItem> =
        serde_json::from_str(json).expect("pinterest live deserializes");
    let item = parsed.first_result().expect("has a result");
    assert_eq!(item.pins_count, Some(1280));
    assert_eq!(
        item.page_url.as_deref(),
        Some("https://www.example.com/recipe")
    );
}

/// Live tests hit the real DataForSEO API and require `ID` / `PASSWORD` in `.env`.
#[cfg(test)]
mod live {
    use data_for_seo::DataForSeoClient;
    use data_for_seo::entity::BusinessDataApiBusinessListingsSearchRequest;
    use std::env;

    fn client() -> DataForSeoClient {
        dotenv::dotenv().ok();
        let id = env::var("ID").unwrap();
        let pass = env::var("PASSWORD").unwrap();
        DataForSeoClient::new(id, pass)
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn business_listings_search_live() {
        let client = client();
        let mut request = BusinessDataApiBusinessListingsSearchRequest::new();
        request.categories = Some(vec!["coffee_shop".to_string()]);
        request.location_coordinate = Some("53.476225,-2.243572,10".to_string());
        request.limit = Some(1);
        let response = client
            .business_data()
            .business_listings()
            .search_live(vec![request])
            .await
            .unwrap();
        for result in response.results() {
            println!("total_count: {:?}", result.total_count);
        }
    }
}
