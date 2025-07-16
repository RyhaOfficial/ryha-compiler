// ryha-toolchain/ryha/src/gemini.rs

use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize)]
struct Candidate {
    content: Content,
}

pub struct GeminiClient {
    api_key: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        GeminiClient { api_key }
    }

    pub async fn explain(&self, error: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: format!("Explain this compiler error: {}", error),
                }],
            }],
        };

        let response: GeminiResponse = client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
                self.api_key
            ))
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.candidates[0].content.parts[0].text.clone())
    }
}
