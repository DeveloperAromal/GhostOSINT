use crate::utils::llm::llm;
use crate::agent::prompt::query_builder_prompt;

pub async fn query_generator(ch_sketch: &str){

    let prompt = query_builder_prompt(ch_sketch);
    let res = llm(&prompt).await.unwrap();

    println!("{}", res);

}