use data_for_seo::entity::SerpApiGoogleOrganicItem;
use data_for_seo::{
    DataForSeoClient, SerpApiGoogleJobsTaskPostRequest, SerpApiGoogleOrganicTaskPostRequest,
};
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();

    DataForSeoClient::new(id, pass)
}

#[tokio::test]
async fn test_locations() {
    let client = client();
    let tasks = client.serp().google().locations().await.unwrap().tasks;
    assert_ne!(tasks.len(), 0);
    for task in tasks {
        println!("{:}", task.id);
        let results = task.result.unwrap();
        assert_ne!(results.len(), 0);
        for result in results {
            if result.clone().location_name.unwrap().as_str() == "Japan" {
                println!("{:?}", result);
                return;
            }
        }
    }
}
#[tokio::test]
async fn test_location_country() {
    let client = client();
    let tasks = client
        .serp()
        .google()
        .locations_country("JP")
        .await
        .unwrap()
        .tasks;
    assert_ne!(tasks.len(), 0);
    for task in tasks {
        println!("{:}", task.id);
        let results = task.result.unwrap();
        assert_ne!(results.len(), 0);
        println!("{:?}", results);
    }
}
#[tokio::test]
async fn test_language() {
    let client = client();
    let tasks = client.serp().google().languages().await.unwrap().tasks;
    assert_ne!(tasks.len(), 0);
    for task in tasks {
        println!("{:}", task.id);
        let results = task.result.unwrap();
        assert_ne!(results.len(), 0);
        for result in results {
            println!("{:?}", result);
        }
    }
}
#[tokio::test]
async fn test_fixed() {
    let client = client();
    let tasks = client
        .serp()
        .google()
        .organic_tasks_fixed()
        .await
        .unwrap()
        .tasks;
    assert_ne!(tasks.len(), 0);
    for task in tasks {
        println!("{:?}", task);
        // let results = task.result.unwrap();
        // assert_ne!(results.len(), 0);
        // for result in results {
        //     println!("{:?}", result);
        // }
    }
}
#[tokio::test]
async fn test() {
    let client = client();

    let mut request = SerpApiGoogleOrganicTaskPostRequest::new("ja".to_string(), 1009307);

    request.keyword = "介護".to_string();
    let a = client
        .serp()
        .google()
        .organic_task_post(vec![request])
        .await
        .unwrap();

    let id = "xxx";
    let a = client
        .serp()
        .google()
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
                    SerpApiGoogleOrganicItem::Unknown(v) => {
                        println!("Unknown");
                    }
                    _ => {
                        println!("other");
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn job_location_country() {
    let client = client();
    let tasks = client.serp().google().jobs_locations().await.unwrap().tasks;
    assert_ne!(tasks.len(), 0);
    for task in tasks {
        println!("{:}", task.id);
        let results = task.result.unwrap();
        assert_ne!(results.len(), 0);
        println!("{:?}", results);
    }
}

#[tokio::test]
async fn job() {
    let client = client();

    let mut request = SerpApiGoogleJobsTaskPostRequest::new("ja".to_string(), 1009307);
    // //
    // request.keyword = "求人 大阪府".to_string();
    // request.priority = Some(2);
    // request.depth=Some(100);
    // let a = client.serp().google().jobs_task_post(vec![request]).await.unwrap();
    // dbg!(a);
    let id = "xxx";
    let a = client
        .serp()
        .google()
        .jobs_task_get_advanced(id)
        .await
        .unwrap();
    let b = client.serp().google().jobs_task_get_html(id).await.unwrap();

    for task in a.tasks {
        for result in task.result.unwrap() {
            println!("{}", result.keyword);
            for item in result.items.unwrap() {
                println!(
                    "{:?} {:?} {:?} {:?}",
                    item.rank_group, item.title, item.source_name, item.employer_name
                );
            }
        }
    }
}

#[cfg(test)]
mod news {
    use crate::client;

    #[tokio::test]
    async fn post() {
        let client = client();
        // let mut request = SerpApiGoogleNewsTaskPostRequest::new("ja".to_string(), 1009307);
        // request.keyword = "介護".to_string();
        // request.priority = Some(2);
        // let res = client.serp().google().news_task_post(vec![request]).await;;
        // println!("{:?}",res);
        let id = "12130243-8697-0066-0000-966d2a98fb1c";
        let res = client
            .serp()
            .google()
            .news_task_get_advanced(id)
            .await
            .unwrap();
        for task in res.tasks {
            for result in task.result.unwrap() {
                println!("{:?}", result.keyword);
                for item in result.items.unwrap() {
                    println!("{:?}*", item);
                }
            }
        }
    }
}
#[cfg(test)]
mod events {
    use crate::client;
    use data_for_seo::SerpApiGoogleEventsTaskPostRequest;

    #[tokio::test]
    async fn example() {
        let client = client();
        let res = client.serp().google().events_locations().await;
        // for v in res.unwrap().tasks.first().unwrap().result{
        //     println!("{:?}", v);
        //
        // }
        // let mut request = SerpApiGoogleEventsTaskPostRequest::new("ja".to_string(), 1009307);
        // // let mut request = SerpApiGoogleEventsTaskPostRequest::new("ja".to_string(), 2392);
        // request.keyword = "転職フェア".to_string();
        // request.priority = Some(2);
        // let res = client.serp().google().events_task_post(vec![request]).await;;
        // println!("{:?}",res);
        // // let id = "12130301-8697-0066-0000-c4418eb093ee";
        // // let res = client.serp().google().events_task_get_advanced(id).await.unwrap();
        // // for task in res.tasks {
        // //     for result in task.result.unwrap() {
        // //         println!("{:?}", result.keyword);
        // //         for item in result.items.unwrap() {
        // //             println!("{:?}*", item);
        // //         }
        // //     }
        // // }
    }
}

#[cfg(test)]
mod images {
    use crate::client;
    use data_for_seo::{SerpApiGoogleEventsTaskPostRequest, SerpApiGoogleImagesTaskPostRequest};

    #[tokio::test]
    async fn example() {
        let client = client();
        // let mut request = SerpApiGoogleImagesTaskPostRequest::new("ja".to_string(), 1009307);
        // // let mut request = SerpApiGoogleEventsTaskPostRequest::new("ja".to_string(), 2392);
        // request.keyword = "介護".to_string();
        // request.priority = Some(2);
        // let res = client.serp().google().images_task_post(vec![request]).await;;
        // println!("{:?}",res);
        let id = "12130742-8697-0066-0000-7b63156695db";
        let res = client
            .serp()
            .google()
            .images_task_get_advanced(id)
            .await
            .unwrap();
        for task in res.tasks {
            for result in task.result.unwrap() {
                println!("{:?}", result.keyword);
                for item in result.items.unwrap() {
                    println!("{:?}*", item);
                }
            }
        }
    }
}
#[cfg(test)]
mod autocomplete {
    use crate::client;
    use data_for_seo::{
        SerpApiGoogleAutoCompleteTaskPostRequest, SerpApiGoogleJobsTaskPostRequest,
    };

    #[tokio::test]
    async fn post() {
        let client = client();
        let mut request = SerpApiGoogleAutoCompleteTaskPostRequest::new("ja".to_string(), 1009307);
        request.keyword = "介護".to_string();
        request.priority = Some(2);
        let res = client
            .serp()
            .google()
            .autocomplete_task_post(vec![request])
            .await;
        println!("{:?}", res);
        let id = "xxx";
        let res = client
            .serp()
            .google()
            .autocomplete_task_get_advanced(id)
            .await
            .unwrap();
        for task in res.tasks {
            for result in task.result.unwrap() {
                println!("{:?}", result.keyword);
                for item in result.items.unwrap() {
                    println!(
                        "{:?} {:?} {:?}*",
                        item.suggestion_type, item.suggestion, item
                    );
                }
            }
        }
    }
}
