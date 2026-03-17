
#[derive(Debug, Clone)]
pub struct Settings {
    pub llm_api_key: String,
    pub llm_model: String,
    pub llm_base_url: String,
}

impl Settings {
    pub fn load() -> Self {
        Self {
            llm_api_key: "".to_string(),

            llm_model: "gemini-2.5-flash".to_string(),

            llm_base_url: "https://generativelanguage.googleapis.com/v1beta"
                .to_string(),
        }
    }
}