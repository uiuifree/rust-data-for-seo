//! Bing SERP API tests.

use data_for_seo::entity::{
    DataForSeoApiResponseData, SerpApiBingOrganicTaskAdvanced, SerpApiGoogleOrganicItem,
};
use data_for_seo::{DataForSeoClient, SerpApiBingOrganicTaskPostRequest};
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();
    DataForSeoClient::new(id, pass)
}

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn organic_task_post() {
    let client = client();
    let mut request = SerpApiBingOrganicTaskPostRequest::new("ja".to_string(), 1009307);
    request.keyword = "SEO".to_string();
    let posted = client
        .serp()
        .bing()
        .organic_task_post(vec![request])
        .await
        .unwrap();
    for task in posted.tasks() {
        println!("posted bing task {}", task.id);
    }
}

#[test]
fn deserialize_bing_organic_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.1 sec",
        "cost": 0.002,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011543-1535-0067-0000-17d3dd1a1b2f",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec",
            "cost": 0.002,
            "path": ["v3", "serp", "bing", "organic", "task_get", "advanced"],
            "data": {"api": "serp"},
            "result": [{
                "keyword": "seo",
                "type": "organic",
                "se_domain": "bing.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://www.bing.com/search?q=seo",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "item_types": ["organic"],
                "se_results_count": 42,
                "items_count": 1,
                "items": [
                    {
                        "type": "organic",
                        "rank_group": 1,
                        "rank_absolute": 1,
                        "domain": "bing-example.com",
                        "title": "Bing Example",
                        "url": "https://bing-example.com/"
                    }
                ]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<SerpApiBingOrganicTaskAdvanced> =
        serde_json::from_str(json).expect("bing response should deserialize");

    let result = response.first_result().expect("one result");
    assert_eq!(result.keyword, "seo");

    let items = result.items.as_ref().expect("items present");
    match &items[0] {
        SerpApiGoogleOrganicItem::Organic(organic) => {
            assert_eq!(organic.domain.as_deref(), Some("bing-example.com"));
        }
        other => panic!("expected organic item, got {other:?}"),
    }
}
