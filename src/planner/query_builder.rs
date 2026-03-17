use crate::utils::llm::llm;
use crate::agent::prompt::query_builder_prompt;

pub async fn query_generator(ch_sketch: &str) -> Vec<String> {

    let prompt = query_builder_prompt(ch_sketch);

    let res = llm(&prompt).await.unwrap();

    let queries: Vec<String> = serde_json::from_str(&res).unwrap_or_default();

    queries
}