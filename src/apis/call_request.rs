use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};
// Call Large language model (i.e GPT-4)

pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    // Extract api key info
    let api_key: String = env::var("OPEN_AI_KEY").expect("Open ai key not found in variables");
    let api_org: String = env::var("OPEN_AI_ORG").expect("Open ai org id not foudn");

    let url: &str = "https://api.openai.com/v1/chat/completions";

    let mut headers = HeaderMap::new();
    // Create api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Create client

    let client = Client::builder().default_headers(headers).build().unwrap();

    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    let res_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
}
