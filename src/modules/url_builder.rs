use serde_json::Value;
use crate::utils::llm::llm;
use crate::agent::prompt::url_builder_prompt;
use crate::utils::clean_output::clean_markdown_to_json;


pub async fn craft_url(search_results: Value) -> Vec<Value> {

    let mut profiles = Vec::new();

    if let Some(results) = search_results["results"].as_array() {

        for r in results {

            if let Some(url) = r["url"].as_str() {

                let prompt = url_builder_prompt(url, None, None);

                match llm(&prompt).await {

                    Ok(res) => {

                        if let Some(cleaned) = clean_markdown_to_json(&res) {
                            profiles.push(cleaned);
                        } else {
                            println!("[-] Invalid JSON from LLM");
                        }

                    }

                    Err(e) => {
                        println!("[-] LLM error: {}", e);
                    }
                }
            }
        }
    }

    profiles
}