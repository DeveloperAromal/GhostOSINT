use reqwest;
use scraper::{Html, Selector};

pub async fn search(query: &str) -> Result<Vec<(String, String, String)>, reqwest::Error> {
    
    let encoded = query.replace(" ", "+");
    let url = format!("https://html.duckduckgo.com/html/?q={}", encoded);

    let client = reqwest::Client::new();
    let html = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "text/html")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html);

    let title_sel   = Selector::parse(".result__title a").unwrap();
    let snippet_sel = Selector::parse(".result__snippet").unwrap();
    let url_sel     = Selector::parse(".result__url").unwrap();

    let titles:   Vec<_> = document.select(&title_sel).collect();
    let snippets: Vec<_> = document.select(&snippet_sel).collect();
    let urls:     Vec<_> = document.select(&url_sel).collect();

    let results: Vec<(String, String, String)> = titles.iter().enumerate().map(|(i, t)| {
        let title   = t.text().collect::<String>().trim().to_string();
        let snippet = snippets.get(i).map(|s| s.text().collect::<String>().trim().to_string()).unwrap_or_default();
        let url     = urls.get(i).map(|u| u.text().collect::<String>().trim().to_string()).unwrap_or_default();
        (title, url, snippet)
    }).collect();

    for (title, url, snippet) in &results {
        println!("Title:   {}", title);
        println!("URL:     {}", url);
        println!("Snippet: {}\n", snippet);
    }

    Ok(results)
}