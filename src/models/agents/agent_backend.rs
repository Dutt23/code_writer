use super::agent_traits::{FactSheet, SpecialFunctions};
use crate::{
    ai_functions::aifunc_backend::{
        print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
        print_rest_api_endpoints,
    },
    helpers::{
        command_line::{confirm_safe_code, PrintCommand},
        general::{
            ai_task_request, read_code_template_contents, read_exec_main_contents,
            save_backend_code,
        },
    },
    models::agent_basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_traits::BasicTrait,
    },
};
use dotenv::dotenv;

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes = BasicAgent::new(
            "Develops backend code for websever and json database".to_string(),
            "Backend developer".to_string(),
        );
        Self {
            attributes,
            bug_count: 0,
            bug_errors: None,
        }
    }

    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();

        let msg_context: String = format!(
            "CODE TEMPLATE {} \n PROJECT_DESCRIPTION {} \n",
            code_template_str, factsheet.project_description
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let msg_context: String = format!(
            "BROKEN_CODE {:?} \n ERROR_BUGS {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.
            ",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_extract_rest_api_endpoints(&self) -> String {
        let backend_code = read_exec_main_contents();

        let msg_context: String = format!("CODE_INPUT: {}", backend_code);
        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;
        ai_response
    }
}

#[async_trait::async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match &self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.state = AgentState::Working;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await;
                    } else {
                        self.call_fix_code_bugs(factsheet).await;
                    }
                    self.attributes.state = AgentState::UnitTesting;
                    continue;
                }
                AgentState::UnitTesting => {
                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position,
                        "Backend code unit testing, ensuring safe code",
                    );

                    let is_safe_code = confirm_safe_code();

                    if !is_safe_code {
                        panic!("Exiting process .....")
                    }
                    self.attributes.state = AgentState::Finished;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_writing_backend_code() {
        std::env::set_var("RUST_BACKTRACE", "1");
        dotenv().ok();
        let mut agent = AgentBackendDeveloper::new();
        let fact_sheet_str: &str = r#"{
    "project_description": "build a website that fetches and tracks fitness progress including timezone from the web.",
    "project_scope": {
            "is_crud_required": true,
            "is_user_login_and_logout": true,
            "is_external_urls_required": true
        },
    "external_url":
        [
            "https://worldtimeapi.org/api/timezone"
        ],
    "backend_code": null,
    "api_endpoint_schema": null
}"#;
        let mut fact_sheet: FactSheet = serde_json::from_str(fact_sheet_str).unwrap();

        agent
            .execute(&mut fact_sheet)
            .await
            .expect("Failed to execute backend developer agent");
        dbg!(fact_sheet);
    }
}
