use crate::models::{
    agent_basic::basic_agent::BasicAgent,
    agents::agent_traits::{FactSheet, SpecialFunctions},
};

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}
