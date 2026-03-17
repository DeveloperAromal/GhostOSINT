use std::borrow::Cow;

use regex::Regex;
use serde_json::Value;

pub fn clean_markdown_to_json(input: &str) -> Option<Value> {

    let re = Regex::new(r"```[a-zA-Z]*|```").unwrap();

    let cleaned = re.replace_all(input, "");

    match serde_json::from_str(cleaned.trim()) {
        Ok(v) => Some(v),
        Err(_) => None
    }
}