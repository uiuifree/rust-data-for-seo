# rust-data-for-seo

`rust-data-for-seo` is a client library designed to make interacting with the [DataForSEO API](https://dataforseo.com/?aff=183967) simple and efficient in Rust. This library helps you access search engine and market data while leveraging Rust's speed and safety features.


# SerpAPI
## Google  
- [x] Organic
- [x] Maps 
- [x] LocalFinder 
- [x] News 
- [x] Events 
- [x] Images 
- [x] SearchByImage 
- [x] Jobs 
- [x] Autocomplete 
- [x] DatasetInfo 
- [x] AdsAdvertises 
- [x] AdsSearch 

Example
```rust
fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();

    DataForSeoClient::new(id, pass)
}



#[tokio::test]
async fn post() {
    let client = client();
    let mut request = KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest::new("ja".to_string(), 20636);
    request.keywords = vec!["SEO".to_string()];
    request.search_partners = Some(true);
    let res = client
        .keywords_data()
        .google_ads()
        .search_volume_task_post(vec![request])
        .await;
    println!("{:?}", res);
}
```


TODO 
Bing
YouTube
Yahoo
Baidu
Naver
Seznam


# Keyword Data API
## Google
- [x] Search Volume
- [x] Keywords For Site
- [x] Keywords For Keywords
- [x] Ad Traffic By Keywords

# Domain Analytics API
TODO

# DataForSEO Labs API
TODO

# Backlinks API
TODO


# OnPage API
- [ ] ID List 
- [ ] Errors
- [ ] Force Stop
- [ ] Filters and Thresholds
- [ ] Task Post
- [ ] Tasks Ready
- [ ] Summary
- [ ] Pages
- [ ] Pages By Resource
- [ ] Resource
- [ ] Duplicate Tags
- [ ] Duplicate Content
- [ ] Links
- [ ] Redirect Chains
- [ ] Non-Indexable
- [ ] Waterfall
- [ ] Keyword Density
- [ ] Microdata
- [ ] Raw HTML
- [ ] Page Screenshot
- [ ] Content Parsing
- [ ] Content Parsing(Live)
- [ ] Instant Pages(Live)
- [ ] Lighthouse

TODO
Content Analysis API
Content Generation API
Merchant API
App Data API
Business Data API
