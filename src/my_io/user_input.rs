use std::io::{stdin, stdout, Write};
pub fn get_user_input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Please enter a correct string");

    s
}