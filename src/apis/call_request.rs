use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};
// Call Large language model (i.e GPT-4)

//used so that it can hold any object which holds the error trait
// dyn used for dynamic dispatch, decides which trait to use at runtime
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract api key info
    let api_key: String = env::var("OPEN_AI_KEY").expect("Open ai key not found in variables");
    let api_org: String = env::var("OPEN_AI_ORG").expect("Open ai org id not found");

    let url: &str = "https://api.openai.com/v1/chat/completions";

    let mut headers = HeaderMap::new();
    // Create api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    // let res_raw = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await
    //     .unwrap();

    // dbg!(res_raw.text().await.unwrap());
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message: Message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response".to_string(),
        };

        let messages: Vec<Message> = vec![message];

        let res = call_gpt(messages).await;

        // if let (Ok(res_string)) = res {
        //     dbg!(res_string);
        //     assert!(true)
        // } else {
        //     assert!(false)
        // }

        match res {
            Ok(res_string) => {
                dbg!(res_string);
                assert!(true)
            }
            Err(_) => assert!(false),
        }
    }
}
