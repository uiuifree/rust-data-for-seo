use data_for_seo::{
    DataForSeoClient,
};
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();

    DataForSeoClient::new(id, pass)
}


#[cfg(test)]
mod search_volume {
    use crate::client;
    use data_for_seo::{KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest, SerpApiGoogleAutoCompleteTaskPostRequest, SerpApiGoogleJobsTaskPostRequest};

    #[tokio::test]
    async fn post() {
        let client = client();
        // Japan ALL
        // let mut request = KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest::new("ja".to_string(), 20636);
        // request.keywords = vec!["介護".to_string()];
        // request.search_partners = Some(true);
        // let res = client
        //     .keywords_data()
        //     .google_ads()
        //     .search_volume_task_post(vec![request])
        //     .await;
        // println!("{:?}", res);
        //
        // return;
        // let id = "xxx";
        // in partner
        let id = "xxx";
        let res = client
            .keywords_data()
            .google_ads().search_volume_task_get(id)
            .await
            .unwrap();
        for task in res.tasks {
            for result in task.result.unwrap() {
                println!("{:?}", result);
            }
        }
    }
}

#[cfg(test)]
mod keyword_for_site {
    use crate::client;
    use data_for_seo::{KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest, KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest, SerpApiGoogleAutoCompleteTaskPostRequest, SerpApiGoogleJobsTaskPostRequest};

    #[tokio::test]
    async fn post() {
        let client = client();
        // // Japan ALL
        // let mut request = KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest::new("ja".to_string(), 20636);
        // request.target = Some("www.google.com".to_string());
        // request.target_type = Some("site".to_string());
        // request.date_from = Some("2024-11-01".to_string());
        // request.date_to = Some("2024-12-01".to_string());
        //

        let id = "xxx";
        let res = client
            .keywords_data()
            .google_ads().keywords_for_site_task_get(id)
            .await
            .unwrap();
        for task in res.tasks {
            println!("{:?}",task.data);
            for result in task.result.unwrap() {
                // println!("{:?}", result);


            }
        }
    }
}
