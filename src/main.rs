mod utils;
mod modules;

#[tokio::main]
async fn main() {
    utils::banner::print_banner();
	modules::websearch::search("DeveloperAromal cybersecurity AI ML software engineer").await.unwrap();
}