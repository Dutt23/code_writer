use crate::{
    ai_functions::aifunc_managing::convert_user_input_to_goal,
    helpers::general::ai_task_request,
    models::{
        agent_basic::basic_agent::{AgentState, BasicAgent},
        agents::agent_traits::{FactSheet, SpecialFunctions},
    },
};

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position: String = "Project Manager".to_string();
        let attributes: BasicAgent = BasicAgent {
            memory: vec![],
            objective: "Manage agents who are building an execellent website for the user"
                .to_string(),
            position: position.clone(),
            state: AgentState::Discovery,
        };

        let project_description: String = ai_task_request(
            usr_req,
            &position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await;

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];
        let factsheet: FactSheet = FactSheet {
            project_description,
            project_scope: None,
            external_url: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        Ok(Self {
            agents,
            attributes,
            factsheet,
        })
    }
}
