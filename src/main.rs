mod config;
mod utils;
mod agent;
mod planner;
mod modules;


#[tokio::main]
async fn main() {
    utils::banner::print_banner();
    modules::websearch::search(r#""DeveloperAromal" site:github.com OR site:gitlab.com OR site:bitbucket.org"#).await.unwrap();   
    // planner::query_builder::query_generator("DeveloperAromal cybersecurity AI ML software engineer").await;
}