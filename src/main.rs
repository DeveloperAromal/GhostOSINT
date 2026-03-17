mod config;
mod utils;
mod agent;
mod planner;
mod modules;

#[tokio::main]
async fn main() {

    utils::banner::print_banner();

    let queries = planner::query_builder::query_generator(
        "DeveloperAromal cybersecurity AI ML software engineer"
    ).await;

    for query in queries {

        println!("\n[*] Searching: {}", query);

        let ws_det = modules::websearch::search(&query).await.unwrap();

        if let Some(results) = ws_det.get("results") {

            if !results.as_array().unwrap().is_empty() {

                let profiles = modules::url_builder::craft_url(ws_det).await;

                for profile in profiles {
                    println!("{}", profile);
                }

            }
        }
    }
}