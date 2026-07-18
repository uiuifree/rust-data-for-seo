use data_for_seo::DataForSeoClient;
use data_for_seo::entity::{AppendixApiUserData, DataForSeoApiResponseData};
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();
    DataForSeoClient::new(id, pass)
}

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn user_data() {
    let client = client();
    let response = client.appendix().user_data().await.unwrap();
    let user = response.first_result().unwrap();
    println!(
        "login: {:?} balance: {:?}",
        user.login,
        user.money.as_ref().and_then(|m| m.balance)
    );
}

#[test]
fn deserialize_user_data() {
    let json = r#"{
        "version": "0.1.20250526",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0 sec.",
        "cost": 0,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "07180000-0000-0000-0000-000000000000",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0 sec.",
            "cost": 0,
            "path": ["v3", "appendix", "user_data"],
            "data": {"api": "appendix", "function": "user_data"},
            "result": [{
                "login": "example@example.com",
                "timezone": "Asia/Tokyo",
                "rates": {
                    "limits": {"day": 2000000, "minute": 2000},
                    "statistics": {"day": {"serp": 10}, "minute": {"serp": 1}}
                },
                "money": {
                    "total": 100.5,
                    "balance": 42.25,
                    "limits": {"day": 0, "minute": 0},
                    "statistics": {"day": {"serp": 0.02}}
                },
                "price": {"serp": {"organic": {"tasks_high": {"cost_type": "per_request", "cost": 0.002}}}},
                "backlinks_subscription_expiry_date": null,
                "llm_mentions_subscription_expiry_date": "2026-12-31 23:59:59 +00:00"
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<AppendixApiUserData> =
        serde_json::from_str(json).expect("user_data response should deserialize");
    let user = response.first_result().expect("one result");
    assert_eq!(user.login.as_deref(), Some("example@example.com"));
    assert_eq!(user.money.as_ref().unwrap().balance, Some(42.25));
    assert!(user.rates.as_ref().unwrap().limits.is_some());
    assert!(user.backlinks_subscription_expiry_date.is_none());
    assert_eq!(
        user.llm_mentions_subscription_expiry_date.as_deref(),
        Some("2026-12-31 23:59:59 +00:00")
    );
}
