use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

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
