use helpers::command_line::get_user_response;
use models::agent_managers::managing_agent::ManagingAgent;

#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

#[tokio::main]
async fn main() {
    let user_req: String = get_user_response("What webserver are we building today ? \n");

    let mut manage_agent: ManagingAgent = ManagingAgent::new(user_req)
        .await
        .expect("Error creating managing agent");
    manage_agent.execute_project().await;

    dbg!(manage_agent);
}
