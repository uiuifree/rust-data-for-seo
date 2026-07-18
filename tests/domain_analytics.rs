//! Tests for the Domain Analytics API domain.
//!
//! The deserialization tests use JSON literals shaped after the DataForSEO
//! documentation samples and require no credentials. Live tests hit the real
//! API and are ignored by default.

use data_for_seo::entity::{
    DataForSeoApiResponseData, DomainAnalyticsApiAggregationTechnologies,
    DomainAnalyticsApiAvailableTechnologies, DomainAnalyticsApiDomainTechnologyItem,
    DomainAnalyticsApiDomainsByTechnology, DomainAnalyticsApiTechnologyStats,
    DomainAnalyticsApiWhoisOverview,
};

#[test]
fn deserialize_whois_overview() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.02,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012345",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.02,
            "result_count": 1,
            "path": ["v3", "domain_analytics", "whois", "overview", "live"],
            "data": {"api": "domain_analytics", "function": "overview"},
            "result": [{
                "total_count": 1234,
                "items_count": 1,
                "offset": 0,
                "offset_token": "token-abc",
                "items": [{
                    "type": "domain_whois_overview",
                    "domain": "example.com",
                    "created_datetime": "1995-08-14 04:00:00 +00:00",
                    "changed_datetime": "2023-08-14 04:00:00 +00:00",
                    "expiration_datetime": "2025-08-13 04:00:00 +00:00",
                    "updated_datetime": "2024-01-01 00:00:00 +00:00",
                    "first_seen": "2019-01-01 00:00:00 +00:00",
                    "epp_status_codes": ["clientDeleteProhibited"],
                    "tld": "com",
                    "registered": true,
                    "registrar": "Example Registrar, Inc.",
                    "metrics": {
                        "organic": {
                            "pos_1": 10,
                            "pos_2_3": 20,
                            "pos_4_10": 30,
                            "etv": 1234.5,
                            "count": 60,
                            "estimated_paid_traffic_cost": 999.9
                        },
                        "paid": {
                            "pos_1": 1,
                            "etv": 12.5,
                            "count": 2
                        }
                    },
                    "backlinks_info": {
                        "referring_domains": 500,
                        "referring_main_domains": 450,
                        "referring_pages": 6000,
                        "dofollow": 5500,
                        "backlinks": 7000,
                        "time_update": "2024-01-01 00:00:00 +00:00"
                    }
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<DomainAnalyticsApiWhoisOverview> =
        serde_json::from_str(json).expect("whois overview response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.total_count, Some(1234));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.domain.as_deref(), Some("example.com"));
    assert_eq!(item.tld.as_deref(), Some("com"));
    assert_eq!(item.registered, Some(true));
    let organic = item.metrics.as_ref().unwrap().organic.as_ref().unwrap();
    assert_eq!(organic.pos_1, Some(10));
    assert_eq!(organic.etv, Some(1234.5));
    assert_eq!(
        item.backlinks_info.as_ref().unwrap().referring_domains,
        Some(500)
    );
}

#[test]
fn deserialize_domain_technologies() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.02,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012345",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.02,
            "result_count": 1,
            "path": ["v3", "domain_analytics", "technologies", "domain_technologies", "live"],
            "data": {"api": "domain_analytics", "target": "example.com"},
            "result": [{
                "type": "domain_technology_item",
                "domain": "example.com",
                "title": "Example Domain",
                "description": "An example website",
                "meta_keywords": ["example", "domain"],
                "domain_rank": 812,
                "last_visited": "2024-01-01 00:00:00 +00:00",
                "country_iso_code": "US",
                "language_code": "en",
                "content_language_code": "en",
                "phone_numbers": ["+1-555-0100"],
                "emails": ["info@example.com"],
                "social_graph_urls": ["https://twitter.com/example"],
                "technologies": {
                    "content": {"cms": ["WordPress"]},
                    "analytics": ["Google Analytics"]
                }
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<DomainAnalyticsApiDomainTechnologyItem> =
        serde_json::from_str(json).expect("domain_technologies response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.domain.as_deref(), Some("example.com"));
    assert_eq!(result.domain_rank, Some(812));
    assert_eq!(result.country_iso_code.as_deref(), Some("US"));
    assert_eq!(result.emails.as_ref().unwrap()[0], "info@example.com");
    assert!(result.technologies.is_some());
}

#[test]
fn deserialize_domains_by_technology() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.02,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012345",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.02,
            "result_count": 1,
            "path": ["v3", "domain_analytics", "technologies", "domains_by_technology", "live"],
            "data": {"api": "domain_analytics", "technologies": ["Shopify"]},
            "result": [{
                "total_count": 90000,
                "items_count": 1,
                "offset": 0,
                "offset_token": "token-xyz",
                "items": [{
                    "type": "domain_technology_item",
                    "domain": "shop.example.com",
                    "title": "Example Shop",
                    "domain_rank": 640,
                    "country_iso_code": "GB",
                    "language_code": "en",
                    "technologies": {"ecommerce": ["Shopify"]}
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<DomainAnalyticsApiDomainsByTechnology> =
        serde_json::from_str(json).expect("domains_by_technology response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.total_count, Some(90000));
    assert_eq!(result.offset_token.as_deref(), Some("token-xyz"));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.domain.as_deref(), Some("shop.example.com"));
    assert_eq!(item.domain_rank, Some(640));
}

#[test]
fn deserialize_aggregation_technologies() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.02,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012345",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.02,
            "result_count": 1,
            "path": ["v3", "domain_analytics", "technologies", "aggregation_technologies", "live"],
            "data": {"api": "domain_analytics", "technology": "Shopify"},
            "result": [{
                "total_count": 3,
                "items_count": 1,
                "items": [{
                    "type": "aggregation_technologies_item",
                    "group": "sales",
                    "category": "ecommerce",
                    "technology": "Shopify",
                    "groups_count": 120000,
                    "categories_count": 95000,
                    "technologies_count": 90000
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<DomainAnalyticsApiAggregationTechnologies> =
        serde_json::from_str(json).expect("aggregation_technologies response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.total_count, Some(3));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.technology.as_deref(), Some("Shopify"));
    assert_eq!(item.group.as_deref(), Some("sales"));
    assert_eq!(item.technologies_count, Some(90000));
}

#[test]
fn deserialize_technology_stats() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.02,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012345",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.02,
            "result_count": 1,
            "path": ["v3", "domain_analytics", "technologies", "technology_stats", "live"],
            "data": {"api": "domain_analytics", "technology": "WordPress"},
            "result": [{
                "technology": "WordPress",
                "date_from": "2023-01-01",
                "date_to": "2023-03-01",
                "items_count": 1,
                "items": [{
                    "type": "technology_stats_item",
                    "date": "2023-01-01 00:00:00 +00:00",
                    "domains_count": 1500000,
                    "countries": {"US": 500000, "GB": 120000},
                    "languages": {"en": 900000},
                    "domains_rank": {"0_100": 100, "101_200": 200}
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<DomainAnalyticsApiTechnologyStats> =
        serde_json::from_str(json).expect("technology_stats response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.technology.as_deref(), Some("WordPress"));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.domains_count, Some(1500000));
    assert_eq!(item.countries.as_ref().unwrap().get("US"), Some(&500000));
}

#[test]
fn deserialize_available_technologies() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.5 sec.",
        "cost": 0.0,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012345",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.0,
            "result_count": 1,
            "path": ["v3", "domain_analytics", "technologies", "technologies"],
            "data": {"api": "domain_analytics"},
            "result": [{
                "sales": {
                    "ecommerce": ["Shopify", "Magento", "WooCommerce"]
                },
                "marketing": {
                    "crm": ["Salesforce", "HubSpot"]
                }
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<DomainAnalyticsApiAvailableTechnologies> =
        serde_json::from_str(json).expect("available_technologies response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    let sales = result.groups.get("sales").expect("sales group expected");
    assert_eq!(
        sales.get("ecommerce").expect("ecommerce category expected"),
        &vec![
            "Shopify".to_string(),
            "Magento".to_string(),
            "WooCommerce".to_string()
        ]
    );
    assert!(result.groups.contains_key("marketing"));
}

#[cfg(test)]
mod live {
    use data_for_seo::{
        DataForSeoClient, DomainAnalyticsApiDomainTechnologiesPost,
        DomainAnalyticsApiWhoisOverviewPost,
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
    async fn available_technologies() {
        let client = client();
        let response = client
            .domain_analytics()
            .technologies()
            .available_technologies()
            .await
            .unwrap();
        assert_eq!(response.status_code, 20000);
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn domain_technologies() {
        let client = client();
        let request = DomainAnalyticsApiDomainTechnologiesPost::new("dataforseo.com".to_string());
        let response = client
            .domain_analytics()
            .technologies()
            .domain_technologies(vec![request])
            .await
            .unwrap();
        assert_eq!(response.status_code, 20000);
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn whois_overview() {
        let client = client();
        let mut request = DomainAnalyticsApiWhoisOverviewPost::new();
        request.limit = Some(10);
        let response = client
            .domain_analytics()
            .whois()
            .overview(vec![request])
            .await
            .unwrap();
        assert_eq!(response.status_code, 20000);
    }
}
