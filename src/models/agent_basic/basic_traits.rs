use crate::models::{agent_basic::basic_agent::AgentState, general::llm::Message};

pub trait BasicTrait {
    fn new(objective: String, position: String) -> Self;
    fn update_state(&mut self, new_state: AgentState);
    fn get_objective(&self) -> &String;
    fn get_position(&self) -> &String;
    fn get_memory(&self) -> &Vec<Message>;
}