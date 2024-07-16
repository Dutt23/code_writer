use crate::{
    ai_functions::aifunc_architext::print_project_scope,
    helpers::general::ai_task_request_decoded,
    models::agent_basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_traits::BasicTrait,
    },
};

use super::agent_traits::{FactSheet, ProjectScope};

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
}
