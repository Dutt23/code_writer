use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            PrintCommand::UnitTest => Color::Magenta,
            PrintCommand::Issue => Color::Red,
        };

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        print!("{} \n", agent_statement);

        stdout.execute(ResetColor).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_agent_msg() {
        PrintCommand::AICall.print_agent_message("Managing Agent", "Processing request here");
    }
}
//  Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print the question in specific color

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    print!("");
    print!("{}", question);

    // Reset Color
    stdout.execute(ResetColor).unwrap();

    // Read user input

    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read user response");

    // Trim whitespace and return

    user_response.to_string()
}
