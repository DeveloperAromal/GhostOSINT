use reqwest::{Client, header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT}};
use scraper::{Html, Selector};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

#[derive(Serialize)]
pub struct SearchResult {
    title: String,
    url: String,
    snippet: String,
}

pub async fn search(query: &str) -> Result<serde_json::Value, reqwest::Error> {

    let encoded = query.replace(" ", "+");
    let url = format!("https://html.duckduckgo.com/html/?q={}", encoded);

    let user_agents = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:123.0) Gecko/20100101 Firefox/123.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0"
    ];

    let ua = user_agents.choose(&mut thread_rng()).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(ua).unwrap());
    headers.insert(ACCEPT, HeaderValue::from_static("text/html"));
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9"));

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let html = client
        .get(&url)
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html);

    let title_sel   = Selector::parse(".result__title a").unwrap();
    let snippet_sel = Selector::parse(".result__snippet").unwrap();
    let url_sel     = Selector::parse(".result__url").unwrap();

    let titles: Vec<_> = document.select(&title_sel).collect();
    let snippets: Vec<_> = document.select(&snippet_sel).collect();
    let urls: Vec<_> = document.select(&url_sel).collect();

    let results: Vec<SearchResult> = titles
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let title = t.text().collect::<String>().trim().to_string();

            let snippet = snippets
                .get(i)
                .map(|s| s.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let url = urls
                .get(i)
                .map(|u| u.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            SearchResult {
                title,
                url,
                snippet
            }
        })
        .collect();

    Ok(serde_json::json!({
        "query": query,
        "results": results
    }))
}