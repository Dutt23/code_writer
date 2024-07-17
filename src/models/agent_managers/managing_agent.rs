use crate::{
    ai_functions::aifunc_managing::convert_user_input_to_goal,
    helpers::general::ai_task_request,
    models::{
        agent_basic::basic_agent::{AgentState, BasicAgent},
        agents::{
            agent_architect::AgentSolutionsArchitect,
            agent_backend::AgentBackendDeveloper,
            agent_traits::{FactSheet, SpecialFunctions},
        },
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

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.agents.push(Box::new(AgentSolutionsArchitect::new()));
        self.agents.push(Box::new(AgentBackendDeveloper::new()));
        // Add backend agent
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_res: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.factsheet).await;

            let agent_info = agent.get_attributes_from_agent();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    pub async fn test_managing_agent() {
        let usr_req= "need a full stack app that fetches and tracks my fitness progress. Need to include timezone from the web".to_string();
        let mut managin_agent = ManagingAgent::new(usr_req)
            .await
            .expect("Unable to create managing agent");

        managin_agent.execute_project().await;
        dbg!(managin_agent.factsheet);
    }
}
