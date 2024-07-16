use reqwest::Client;
use serde::de::DeserializeOwned;
use std::env;

use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;
use std::fs;
use std::io::Write;

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);
    dbg!(ai_function_str);
    let msg: String = format!("FUNCTION {} 
    INSTRUCTION: You are a function printer. You ONLY print the result of the functions. Nothing else. No commentary.
    Here is the input to the function {}.
    Print out what the function will return.
    ", ai_function_str, func_input);

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    let func_msg = extend_ai_function(function_pass, &msg_context);
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // LLM Response
    let llm_response_res = call_gpt(vec![func_msg.clone()]).await;

    match llm_response_res {
        Ok(llm_res) => llm_res,
        Err(_) => call_gpt(vec![func_msg])
            .await
            .expect("Failed twice to call gtp"),
    }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_message: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");

    decoded_message
}

pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get code template
pub fn read_code_template_contents() -> String {
    let path = env::var("TEMPLATE_PATH").expect("Code template path not found");
    let template_path = format!("{}/_code_template.rs", path);
    fs::read_to_string(path).expect("Failed to read code template")
}
// Save new backend code
pub fn save_backend_code(contents: &str) {
    let path = env::var("TEMPLATE_PATH").expect("Code template path not found");
    let main_path = format!("{}/main.rs", path);
    fs::write(path, contents.as_bytes());
}
// Save JSON API Endpoint Schema
pub fn save_api_endpoint(contents: &str) {
    let mut data_file = fs::File::open("schemas/api_schemas.json").expect("Unable to open file");
    data_file
        .write(contents.as_bytes())
        .expect("Failed to write api endpoint json paths");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extending_ai_function() {
        let msg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        dbg!(&msg);
        assert_eq!(msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let res = ai_task_request(
            "Build me a webserver for making stock price requests".to_string(),
            "Managing Agent",
            "Defining User requirements",
            convert_user_input_to_goal,
        )
        .await;

        dbg!(res);
    }
}
