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
}
