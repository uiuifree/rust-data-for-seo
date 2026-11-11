//! Offline deserialization tests for the endpoints added to follow the DataForSEO API.

use data_for_seo::entity::*;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

fn parse<T: DeserializeOwned>(result: Value) -> DataForSeoApiResponseData<T> {
    let body = json!({
        "version": "0.1.20261109",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "0.1 sec",
        "cost": 0,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "11111111-1111-1111-1111-111111111111",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec",
            "cost": 0,
            "result_count": 1,
            "path": ["v3"],
            "data": {},
            "result": [result]
        }]
    });
    serde_json::from_value(body).expect("response deserializes")
}

#[test]
fn deserialize_serp_screenshot() {
    let parsed = parse::<SerpApiScreenshot>(json!({
        "items_count": 1,
        "items": [{"image": "https://api.dataforseo.com/cdn/screenshot.png"}]
    }));
    let items = parsed.first_result().unwrap().items.as_ref().unwrap();
    assert!(items[0].image.as_deref().unwrap().ends_with(".png"));
}

#[test]
fn deserialize_serp_ai_summary() {
    let parsed = parse::<SerpApiAiSummary>(json!({
        "items_count": 1,
        "items": [{"summary": "text"}]
    }));
    let items = parsed.first_result().unwrap().items.as_ref().unwrap();
    assert_eq!(items[0].summary.as_deref(), Some("text"));
}

#[test]
fn deserialize_id_list() {
    let parsed = parse::<DataForSeoIdListItem>(json!({
        "id": "11111111-1111-1111-1111-111111111111",
        "url": "https://api.dataforseo.com/v3/serp/google/organic/task_post",
        "datetime_posted": "2026-11-09 00:00:00 +00:00",
        "datetime_done": "2026-11-09 00:01:00 +00:00",
        "status": 20000,
        "cost": 0.0006,
        "metadata": {"keyword": "seo"}
    }));
    assert_eq!(parsed.first_result().unwrap().cost, Some(0.0006));
}

#[test]
fn deserialize_errors() {
    let parsed = parse::<DataForSeoErrorItem>(json!({
        "id": "11111111-1111-1111-1111-111111111111",
        "datetime": "2026-11-09 00:00:00 +00:00",
        "function": "postback_url",
        "error_code": 50000,
        "error_message": "Internal Error.",
        "http_url": "https://example.com/",
        "http_method": "POST",
        "http_code": 500,
        "http_time": 0.5,
        "http_response": "error"
    }));
    assert_eq!(parsed.first_result().unwrap().http_code, Some(500));
}

#[test]
fn deserialize_html_and_regular_results() {
    let html = parse::<SerpApiSearchResult<SerpApiHtmlItem>>(json!({
        "keyword": "seo",
        "items_count": 1,
        "items": [{"page": 1, "date": "2026-11-09 00:00:00 +00:00", "html": "<html></html>"}]
    }));
    assert_eq!(html.first_result().unwrap().items().len(), 1);

    let regular = parse::<SerpApiSearchResult<SerpApiGoogleOrganicItem>>(json!({
        "keyword": "seo",
        "items_count": 1,
        "items": [{"type": "organic", "rank_group": 1, "rank_absolute": 1, "url": "https://example.com/"}]
    }));
    assert!(matches!(
        regular.first_result().unwrap().items()[0],
        SerpApiGoogleOrganicItem::Organic(_)
    ));
}

#[test]
fn deserialize_appendix_status_and_errors() {
    let status = parse::<AppendixApiStatus>(json!({
        "api": "serp",
        "status": "ok",
        "endpoints": [{"endpoint": "live", "status": "ok"}]
    }));
    assert_eq!(
        status.first_result().unwrap().endpoints.as_ref().unwrap()[0]
            .endpoint
            .as_deref(),
        Some("live")
    );

    let error = parse::<AppendixApiError>(json!({"code": 40501, "message": "Invalid Field"}));
    assert_eq!(error.first_result().unwrap().code, Some(40501));
}

#[test]
fn deserialize_labs_available_history() {
    let parsed = parse::<DataForSeoLabsAvailableHistory>(json!({"date": "2026-10-01"}));
    assert_eq!(
        parsed.first_result().unwrap().date.as_deref(),
        Some("2026-10-01")
    );
}

#[test]
fn deserialize_labs_historical_search_volume() {
    let parsed = parse::<DataForSeoLabsResult<DataForSeoLabsKeywordData>>(json!({
        "se_type": "google",
        "location_code": 2840,
        "language_code": "en",
        "items_count": 1,
        "items": [{
            "se_type": "google",
            "keyword": "seo",
            "location_code": 2840,
            "language_code": "en",
            "keyword_info": {"search_volume": 100}
        }]
    }));
    assert!(parsed.first_result().is_some());
}

#[test]
fn deserialize_on_page_uncrawlable_resources() {
    let parsed = parse::<OnPageDataApiUncrawlableResources>(json!({
        "crawl_progress": "finished",
        "crawl_status": {"max_crawl_pages": 10, "pages_in_queue": 0, "pages_crawled": 10},
        "current_offset": 0,
        "total_items_count": 1,
        "items_count": 1,
        "items": [{
            "url": "https://example.com/a.css",
            "reason": "content_type_inconsistency",
            "status_code": 200,
            "fetch_time": "2026-11-09 00:00:00 +00:00",
            "meta": {"content_type": "text/html", "expected_content_types": ["text/css"]}
        }]
    }));
    let items = parsed.first_result().unwrap().items.as_ref().unwrap();
    assert_eq!(
        items[0].meta.as_ref().unwrap().content_type.as_deref(),
        Some("text/html")
    );
}

#[test]
fn deserialize_lighthouse_metadata() {
    let audits =
        parse::<OnPageDataApiLighthouseAudits>(json!({"audits": ["first-contentful-paint"]}));
    assert_eq!(
        audits
            .first_result()
            .unwrap()
            .audits
            .as_ref()
            .unwrap()
            .len(),
        1
    );

    let versions = parse::<OnPageDataApiLighthouseVersions>(json!({
        "availible_versions": [{"version": "12.0.0", "default": true}]
    }));
    let list = versions.first_result().unwrap();
    assert_eq!(
        list.availible_versions.as_ref().unwrap()[0].default,
        Some(true)
    );
}

#[test]
fn deserialize_app_listings() {
    let categories =
        parse::<AppDataApiAppListingsCategory>(json!({"category": "games", "count": 10}));
    assert_eq!(categories.first_result().unwrap().count, Some(10));

    let search = parse::<AppDataApiAppListingsSearchResult>(json!({
        "total_count": 1,
        "count": 1,
        "offset": 0,
        "offset_token": "token",
        "items": [{
            "app_id": "org.example",
            "se_domain": "play.google.com",
            "location_code": 2840,
            "language_code": "en",
            "check_url": "https://play.google.com/",
            "time_update": "2026-11-09T00:00:00Z",
            "item": {"type": "google_play_info_organic", "title": "App"}
        }]
    }));
    assert_eq!(search.first_result().unwrap().total_count, Some(1));
}

#[test]
fn deserialize_technologies_locations() {
    let parsed = parse::<DomainAnalyticsApiTechnologiesLocation>(json!({
        "location_name": "United States",
        "country_iso_code": "US"
    }));
    assert_eq!(
        parsed.first_result().unwrap().country_iso_code.as_deref(),
        Some("US")
    );
}

#[test]
fn deserialize_bing_keyword_performance_locations() {
    let parsed = parse::<KeywordsDataApiBingLocationsAndLanguages>(json!({
        "language_name": "English",
        "language_code": "en",
        "available_locations": [{
            "location_code": 2840,
            "location_name": "United States",
            "country_iso_code": "US",
            "location_type": "Country"
        }]
    }));
    assert_eq!(
        parsed
            .first_result()
            .unwrap()
            .available_locations
            .as_ref()
            .unwrap()[0]
            .location_code,
        Some(2840)
    );
}

#[test]
fn deserialize_live_response_shapes() {
    let local_pack: SerpApiGoogleOrganicItem = serde_json::from_value(json!({
        "type": "local_pack",
        "rank_group": 1,
        "rank_absolute": 1,
        "is_paid": false
    }))
    .unwrap();
    assert!(matches!(local_pack, SerpApiGoogleOrganicItem::LocalPack(_)));

    let organic: SerpApiGoogleOrganicItem = serde_json::from_value(json!({
        "type": "organic",
        "rank_group": 1,
        "rank_absolute": 1,
        "checks": ["is_image"],
        "related_result": [{"type": "related_result", "url": "https://example.com/a"}]
    }))
    .unwrap();
    assert!(matches!(organic, SerpApiGoogleOrganicItem::Organic(_)));

    let lite = parse::<AiOptimizationLlmMentionsMetricsLite>(json!({
        "total_count": 1,
        "items": [{
            "location": 2840,
            "language": "en",
            "platform": "google",
            "metrics": {"mentions": 1, "ai_search_volume": 2}
        }]
    }));
    let items = lite.first_result().unwrap().items.as_ref().unwrap();
    assert_eq!(items[0].metrics.as_ref().unwrap().mentions, Some(1));

    let clickstream = parse::<
        KeywordsDataApiClickstreamResult<KeywordsDataApiClickstreamGlobalSearchVolume>,
    >(json!({
        "items_count": 1,
        "items": [{"keyword": "seo", "search_volume": 10}]
    }));
    let items = clickstream.first_result().unwrap().items.as_ref().unwrap();
    assert_eq!(items[0].search_volume, Some(10));
}
