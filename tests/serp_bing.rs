use data_for_seo::entity::{SerpApiGoogleOrganicItem, TaskStatus};
use data_for_seo::{DataForSeoClient, SerpApiBingOrganicTaskPostRequest, SerpApiGoogleJobsTaskPostRequest, SerpApiGoogleOrganicTaskPostRequest};
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();

    DataForSeoClient::new(id, pass)
}

#[tokio::test]
async fn test1() {
    let client = client();

    let mut request = SerpApiBingOrganicTaskPostRequest::new("ja".to_string(), 1009307);

    request.keyword = "SEO".to_string();
    let a = client
        .serp()
        .bing()
        .organic_task_post(vec![request])
        .await
        .unwrap();

    println!("{:?}",a);

    let id = "xxx";
    let a = client
        .serp()
        .bing()
        .organic_task_get_advanced(id)
        .await
        .unwrap();

    for task in a.tasks {
        for result in task.result.unwrap() {
            println!("{}", result.keyword);
            for item in result.items.unwrap() {
                match item {
                    SerpApiGoogleOrganicItem::Organic(v) => {
                        println!("organic {:?}", v);
                    }
                    SerpApiGoogleOrganicItem::PeopleAlsoAsk(v) => {
                        println!("PeopleAlsoAsk {:?}", v);
                    }
                    SerpApiGoogleOrganicItem::Unknown(v) => {
                        println!("type field: {:?}", v["type"]);
                        println!("Unknown {:?} {:?}", v.get("type"), v);
                    }
                    _ => {
                        println!("other");
                    }
                }
            }
        }
    }
}
