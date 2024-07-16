use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;

use crate::{
    ai_functions::aifunc_architext::{print_project_scope, print_site_urls},
    helpers::{
        command_line::PrintCommand,
        general::{ai_task_request_decoded, check_status_code},
    },
    models::agent_basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_traits::BasicTrait,
    },
};

use super::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

#[derive(Debug)]
pub struct AgentSolutionsArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionsArchitect {
    pub fn new() -> Self {
        let attributes = BasicAgent::new(
            "Gathers information and designs solutions for website".to_string(),
            "Solutions Architect".to_string(),
        );
        Self { attributes }
    }

    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg_context = format!("{:?}", factsheet.project_description);
        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;

        factsheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);
        ai_response
    }

    async fn determine_external_urls(&mut self, factsheet: &mut FactSheet, msg_context: String) {
        let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;

        factsheet.external_url = Some(ai_response);
        self.attributes.state = AgentState::UnitTesting;
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionsArchitect {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }
    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Infinite loop warning
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope = self.call_project_scope(factsheet).await;

                    if project_scope.is_external_urls_required {
                        self.determine_external_urls(
                            factsheet,
                            factsheet.project_description.clone(),
                        )
                        .await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                }
                AgentState::UnitTesting => {
                    let mut exclude_urls: Vec<String> = vec![];

                    let client: Client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    let urls: &Vec<String> = factsheet
                        .external_url
                        .as_ref()
                        .expect("No url object on factsheet");

                    for url in urls {
                        let endpoint_str: String = format!("Testing URL endpoint {}", url);
                        PrintCommand::UnitTest
                            .print_agent_message(&self.attributes.position, endpoint_str.as_str());
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone());
                                }
                            }
                            Err(e) => print!("Error checking {} : {}", url, e),
                        }
                    }

                    if exclude_urls.len() > 0 {
                        let new_urls: Vec<String> = factsheet
                            .external_url
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(&url))
                            .cloned()
                            .collect();

                        factsheet.external_url = Some(new_urls);
                    }

                    self.attributes.state = AgentState::Finished;
                }
                _ => self.attributes.state = AgentState::Finished,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn tests_solution_architect() {
        let mut agent = AgentSolutionsArchitect::new();
        let mut factsheet = FactSheet {
          project_description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
          project_scope: None,
          external_url:None,
          backend_code: None,
          api_endpoint_schema: None,
        };
        agent
            .execute(&mut factsheet)
            .await
            .expect("Unable to execute solutions architect agent");
        assert!(factsheet.project_scope != None);
        assert!(factsheet.external_url.is_some());

        dbg!(agent);
        dbg!(factsheet);
    }
}
