use crate::models::general::llm::Message;
use dotenv::dotenv;
use reqwest::Client;
use std::env;

// Call Large language model (i.e GPT-4)

pub async fn call_gpt(messages: Vec<Message>) {
  dotenv().ok();

  // Extract api key info
  let api_key: String = env::var("OPEN_AI_KEY").expect("Open ai key not found in variables");
  let api_org: String = env::var("OPEN_AI_ORG").expect("Open ai org id not foudn");

  let url: &str = "https://api.openai.com/v1/chat/completions";
}