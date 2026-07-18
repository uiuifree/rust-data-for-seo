//! Tests for the Merchant API domain.
//!
//! The deserialization tests use JSON literals shaped after the DataForSEO
//! documentation samples and require no credentials. Live tests hit the real
//! API and are ignored by default.

use data_for_seo::entity::{
    DataForSeoApiResponseData, MerchantApiAmazonAsin, MerchantApiAmazonProducts,
    MerchantApiAmazonSellers, MerchantApiGoogleProductInfo, MerchantApiGoogleProducts,
    MerchantApiGoogleReviews,
};

#[test]
fn deserialize_amazon_products_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "1.5 sec.",
        "cost": 0.003,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012345",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.3 sec.",
            "cost": 0.003,
            "result_count": 1,
            "path": ["v3", "merchant", "amazon", "products", "task_get", "advanced"],
            "data": {"api": "merchant", "keyword": "iphone"},
            "result": [{
                "keyword": "iphone",
                "type": "products",
                "se_domain": "amazon.com",
                "location_code": 2840,
                "language_code": "en_US",
                "check_url": "https://amazon.com/s?k=iphone",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "spell": null,
                "item_types": ["amazon_serp", "amazon_paid"],
                "se_results_count": 5000,
                "categories": [],
                "items_count": 1,
                "items": [{
                    "type": "amazon_serp",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "position": "left",
                    "xpath": "/html/body/div",
                    "domain": "amazon.com",
                    "title": "Apple iPhone",
                    "url": "https://amazon.com/dp/B0TEST",
                    "image_url": "https://example.com/iphone.jpg",
                    "bought_past_month": 5000,
                    "price_from": 799.0,
                    "price_to": 999.0,
                    "currency": "USD",
                    "special_offers": [],
                    "data_asin": "B0TEST",
                    "rating": {
                        "rating_type": "Max5",
                        "value": 4.7,
                        "votes_count": 1234,
                        "rating_max": 5
                    },
                    "is_amazon_choice": true,
                    "is_best_seller": false,
                    "delivery_info": {
                        "delivery_message": "FREE delivery",
                        "delivery_price": {
                            "current": 0.0,
                            "currency": "USD"
                        },
                        "delivery_date_from": "2024-01-05",
                        "delivery_date_to": "2024-01-07"
                    }
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<MerchantApiAmazonProducts> =
        serde_json::from_str(json).expect("amazon products response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.keyword.as_deref(), Some("iphone"));
    assert_eq!(result.se_domain.as_deref(), Some("amazon.com"));
    assert_eq!(result.items_count, Some(1));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.item_type.as_deref(), Some("amazon_serp"));
    assert_eq!(item.data_asin.as_deref(), Some("B0TEST"));
    assert_eq!(item.price_from, Some(799.0));
    assert_eq!(item.is_amazon_choice, Some(true));
    assert_eq!(item.rating.as_ref().unwrap().value, Some(4.7));
    assert_eq!(
        item.delivery_info
            .as_ref()
            .unwrap()
            .delivery_price
            .as_ref()
            .unwrap()
            .currency
            .as_deref(),
        Some("USD")
    );
}

#[test]
fn deserialize_amazon_asin_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "1.5 sec.",
        "cost": 0.003,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012346",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.3 sec.",
            "cost": 0.003,
            "result_count": 1,
            "path": ["v3", "merchant", "amazon", "asin", "task_get", "advanced"],
            "data": {"api": "merchant", "asin": "B0TEST"},
            "result": [{
                "asin": "B0TEST",
                "type": "product_info",
                "se_domain": "amazon.com",
                "location_code": 2840,
                "language_code": "en_US",
                "check_url": "https://amazon.com/dp/B0TEST",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "spell": null,
                "item_types": ["amazon_product_info"],
                "items_count": 1,
                "items": [{
                    "type": "amazon_product_info",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "position": "left",
                    "xpath": "/html/body/div",
                    "title": "Apple iPhone 15",
                    "details": "Latest model",
                    "image_url": "https://example.com/iphone.jpg",
                    "author": null,
                    "data_asin": "B0TEST",
                    "parent_asin": "B0PARENT",
                    "product_asins": ["B0TEST", "B0VARIANT"],
                    "price_from": 799.0,
                    "price_to": null,
                    "percentage_discount": 10.0,
                    "currency": "USD",
                    "is_amazon_choice": true,
                    "rating": {
                        "rating_type": "Max5",
                        "value": 4.7,
                        "votes_count": 1234,
                        "rating_max": 5
                    },
                    "is_newer_model_available": false,
                    "is_prime_video": false,
                    "applicable_vouchers": [],
                    "newer_model": null,
                    "categories": ["Electronics"],
                    "product_information": [],
                    "product_images_list": ["https://example.com/1.jpg"],
                    "product_videos_list": [],
                    "description": "A great phone.",
                    "is_available": true,
                    "top_local_reviews": [],
                    "top_global_reviews": []
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<MerchantApiAmazonAsin> =
        serde_json::from_str(json).expect("amazon asin response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.asin.as_deref(), Some("B0TEST"));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.item_type.as_deref(), Some("amazon_product_info"));
    assert_eq!(item.parent_asin.as_deref(), Some("B0PARENT"));
    assert_eq!(
        item.product_asins.as_ref().unwrap(),
        &vec!["B0TEST".to_string(), "B0VARIANT".to_string()]
    );
    assert_eq!(item.percentage_discount, Some(10.0));
    assert_eq!(item.is_available, Some(true));
}

#[test]
fn deserialize_amazon_sellers_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "1.5 sec.",
        "cost": 0.003,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012347",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.3 sec.",
            "cost": 0.003,
            "result_count": 1,
            "path": ["v3", "merchant", "amazon", "sellers", "task_get", "advanced"],
            "data": {"api": "merchant", "asin": "B0TEST"},
            "result": [{
                "asin": "B0TEST",
                "type": "sellers",
                "se_domain": "amazon.com",
                "location_code": 2840,
                "language_code": "en_US",
                "check_url": "https://amazon.com/gp/offer-listing/B0TEST",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "title": "Apple iPhone 15",
                "image": "https://example.com/iphone.jpg",
                "item_types": ["amazon_seller_main_item", "amazon_seller_item"],
                "items_count": 1,
                "items": [{
                    "type": "amazon_seller_main_item",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "position": "left",
                    "xpath": "/html/body/div",
                    "seller_name": "Amazon.com",
                    "seller_url": "https://amazon.com/sp",
                    "ships_from": "Amazon.com",
                    "price": {
                        "current": 799.0,
                        "regular": 899.0,
                        "currency": "USD",
                        "is_price_range": false,
                        "displayed_price": "$799.00"
                    },
                    "percentage_discount": 11.0,
                    "applicable_vouchers": [],
                    "rating": {
                        "rating_type": "Max5",
                        "value": 4.5,
                        "votes_count": 500,
                        "rating_max": 5
                    },
                    "condition": "New",
                    "condition_description": null,
                    "delivery_info": {
                        "delivery_message": "FREE delivery"
                    }
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<MerchantApiAmazonSellers> =
        serde_json::from_str(json).expect("amazon sellers response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.asin.as_deref(), Some("B0TEST"));
    assert_eq!(result.title.as_deref(), Some("Apple iPhone 15"));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.seller_name.as_deref(), Some("Amazon.com"));
    assert_eq!(item.condition.as_deref(), Some("New"));
    assert_eq!(item.price.as_ref().unwrap().current, Some(799.0));
    assert_eq!(
        item.price.as_ref().unwrap().displayed_price.as_deref(),
        Some("$799.00")
    );
}

#[test]
fn deserialize_google_products_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "1.5 sec.",
        "cost": 0.003,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012348",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.3 sec.",
            "cost": 0.003,
            "result_count": 1,
            "path": ["v3", "merchant", "google", "products", "task_get", "advanced"],
            "data": {"api": "merchant", "keyword": "shoes"},
            "result": [{
                "keyword": "shoes",
                "type": "products",
                "se_domain": "google.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://google.com/search?tbm=shop&q=shoes",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "spell": null,
                "item_types": ["google_shopping_serp"],
                "se_results_count": 12000,
                "items_count": 1,
                "items": [{
                    "type": "google_shopping_serp",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "position": "left",
                    "xpath": "/html/body/div",
                    "domain": "example-shop.com",
                    "title": "Running Shoes",
                    "description": "Comfortable running shoes",
                    "url": "https://example-shop.com/shoes",
                    "shopping_url": "https://google.com/shopping/product/1",
                    "tags": ["free shipping"],
                    "price": 89.99,
                    "price_multiplier": 1,
                    "old_price": 119.99,
                    "currency": "USD",
                    "product_id": "1234567890",
                    "data_docid": "9876543210",
                    "seller": "Example Shop",
                    "additional_specifications": {"color": "black"},
                    "reviews_count": 250,
                    "is_best_match": true,
                    "product_rating": {
                        "rating_type": "Max5",
                        "value": 4.3,
                        "votes_count": 250,
                        "rating_max": 5
                    },
                    "shop_rating": null,
                    "product_images": ["https://example.com/shoe.jpg"],
                    "shop_ad_aclk": null,
                    "delivery_info": null,
                    "stores_count_info": null,
                    "gid": "gid-abc"
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<MerchantApiGoogleProducts> =
        serde_json::from_str(json).expect("google products response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.keyword.as_deref(), Some("shoes"));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.item_type.as_deref(), Some("google_shopping_serp"));
    assert_eq!(item.product_id.as_deref(), Some("1234567890"));
    assert_eq!(item.price, Some(89.99));
    assert_eq!(item.old_price, Some(119.99));
    assert_eq!(item.is_best_match, Some(true));
    assert_eq!(item.product_rating.as_ref().unwrap().value, Some(4.3));
}

#[test]
fn deserialize_google_product_info_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "1.5 sec.",
        "cost": 0.003,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef012349",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.3 sec.",
            "cost": 0.003,
            "result_count": 1,
            "path": ["v3", "merchant", "google", "product_info", "task_get", "advanced"],
            "data": {"api": "merchant", "product_id": "1234567890"},
            "result": [{
                "product_id": "1234567890",
                "type": "product_info",
                "se_domain": "google.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://google.com/shopping/product/1234567890",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "item_types": ["google_product_info"],
                "items_count": 1,
                "items": [{
                    "type": "google_product_info",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "position": "left",
                    "product_id": "1234567890",
                    "title": "Running Shoes",
                    "description": "Comfortable running shoes",
                    "url": "https://google.com/shopping/product/1234567890",
                    "images": ["https://example.com/shoe.jpg"],
                    "features": [],
                    "rating": {
                        "rating_type": "Max5",
                        "value": 4.3,
                        "votes_count": 250,
                        "rating_max": 5
                    },
                    "seller_reviews_count": 12,
                    "data_docid": "9876543210",
                    "gid": "gid-abc",
                    "specifications": [{
                        "type": "specification",
                        "block_name": "General",
                        "specification_name": "Color",
                        "specification_value": "Black"
                    }],
                    "sellers": [{
                        "type": "seller",
                        "title": "Example Shop",
                        "url": "https://example-shop.com",
                        "seller_rating": null,
                        "seller_reviews_count": 30,
                        "price": {
                            "current": 89.99,
                            "currency": "USD"
                        },
                        "delivery_info": null,
                        "product_availability": "in_stock"
                    }],
                    "variations": [{
                        "type": "variation",
                        "product_id": "1234567891",
                        "gid": "gid-def",
                        "pvf": "size:10",
                        "title": "Running Shoes (Size 10)",
                        "url": "https://google.com/shopping/product/1234567891",
                        "variation_category": "size"
                    }]
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<MerchantApiGoogleProductInfo> =
        serde_json::from_str(json).expect("google product_info response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.product_id.as_deref(), Some("1234567890"));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.title.as_deref(), Some("Running Shoes"));
    let spec = &item.specifications.as_ref().unwrap()[0];
    assert_eq!(spec.specification_name.as_deref(), Some("Color"));
    assert_eq!(spec.specification_value.as_deref(), Some("Black"));
    let seller = &item.sellers.as_ref().unwrap()[0];
    assert_eq!(seller.price.as_ref().unwrap().current, Some(89.99));
    let variation = &item.variations.as_ref().unwrap()[0];
    assert_eq!(variation.pvf.as_deref(), Some("size:10"));
}

#[test]
fn deserialize_google_reviews_advanced() {
    let json = r#"{
        "version": "0.1.20240101",
        "status_code": 20000,
        "status_message": "Ok.",
        "time": "1.5 sec.",
        "cost": 0.003,
        "tasks_count": 1,
        "tasks_error": 0,
        "tasks": [{
            "id": "01011234-1234-0123-0000-abcdef01234a",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.3 sec.",
            "cost": 0.003,
            "result_count": 1,
            "path": ["v3", "merchant", "google", "reviews", "task_get", "advanced"],
            "data": {"api": "merchant", "gid": "gid-abc"},
            "result": [{
                "product_id": "1234567890",
                "type": "reviews",
                "se_domain": "google.com",
                "location_code": 2840,
                "language_code": "en",
                "check_url": "https://google.com/shopping/product/1234567890/reviews",
                "datetime": "2024-01-01 00:00:00 +00:00",
                "spell": null,
                "title": "Running Shoes",
                "image_url": "https://example.com/shoe.jpg",
                "rating": {
                    "rating_type": "Max5",
                    "value": 4.3,
                    "votes_count": 250,
                    "rating_max": 5
                },
                "rating_groups": [],
                "top_keywords": [],
                "reviews_count": 250,
                "item_types": ["google_reviews_search"],
                "items_count": 1,
                "items": [{
                    "type": "google_reviews_search",
                    "rank_group": 1,
                    "rank_absolute": 1,
                    "position": "left",
                    "images": [],
                    "title": "Great shoes",
                    "url": null,
                    "review_text": "Very comfortable, would buy again.",
                    "provided_by": "Example Shop",
                    "author": "Jane D.",
                    "publication_date": "2024-01-01",
                    "rating": {
                        "rating_type": "Max5",
                        "value": 5.0,
                        "votes_count": null,
                        "rating_max": 5
                    }
                }]
            }]
        }]
    }"#;

    let response: DataForSeoApiResponseData<MerchantApiGoogleReviews> =
        serde_json::from_str(json).expect("google reviews response should deserialize");

    assert_eq!(response.status_code, 20000);
    let result = response.first_result().expect("one result expected");
    assert_eq!(result.reviews_count, Some(250));
    assert_eq!(result.rating.as_ref().unwrap().value, Some(4.3));
    let item = &result.items.as_ref().unwrap()[0];
    assert_eq!(item.author.as_deref(), Some("Jane D."));
    assert_eq!(
        item.review_text.as_deref(),
        Some("Very comfortable, would buy again.")
    );
    assert_eq!(item.rating.as_ref().unwrap().value, Some(5.0));
}

#[test]
fn request_skips_none_fields() {
    use data_for_seo::MerchantApiAmazonProductsPost;

    // Only the required `keyword` plus explicitly set fields should serialize;
    // all `None` fields must be skipped via `skip_serializing_if`.
    let mut request = MerchantApiAmazonProductsPost::new("iphone");
    request.location_code = Some(2840);
    request.language_code = Some("en_US".to_string());

    let value = serde_json::to_value(&request).expect("request should serialize");
    let object = value.as_object().expect("request serializes to an object");

    assert_eq!(
        object.get("keyword").and_then(|v| v.as_str()),
        Some("iphone")
    );
    assert_eq!(
        object.get("location_code").and_then(|v| v.as_i64()),
        Some(2840)
    );
    assert_eq!(
        object.get("language_code").and_then(|v| v.as_str()),
        Some("en_US")
    );
    assert!(!object.contains_key("url"));
    assert!(!object.contains_key("priority"));
    assert!(!object.contains_key("depth"));
    assert!(!object.contains_key("tag"));
}

// ---------------------------------------------------------------------------
// Live integration tests (require DataForSEO credentials in a `.env` file).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod live {
    use data_for_seo::DataForSeoClient;
    use data_for_seo::MerchantApiAmazonProductsPost;
    use std::env;

    fn client() -> DataForSeoClient {
        dotenv::dotenv().ok();
        let id = env::var("ID").unwrap();
        let pass = env::var("PASSWORD").unwrap();
        DataForSeoClient::new(id, pass)
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn amazon_locations() {
        let client = client();
        let res = client.merchant().amazon().locations().await.unwrap();
        assert_eq!(res.status_code, 20000);
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn amazon_products_task_post() {
        let client = client();
        let mut request = MerchantApiAmazonProductsPost::new("iphone");
        request.location_code = Some(2840);
        request.language_code = Some("en_US".to_string());
        let res = client
            .merchant()
            .amazon()
            .products_task_post(vec![request])
            .await
            .unwrap();
        assert_eq!(res.status_code, 20000);
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn google_products_tasks_ready() {
        let client = client();
        let res = client
            .merchant()
            .google()
            .products_tasks_ready()
            .await
            .unwrap();
        assert_eq!(res.status_code, 20000);
    }
}
