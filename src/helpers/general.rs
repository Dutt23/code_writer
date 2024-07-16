use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;

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
