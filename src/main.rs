#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

enum Command {
    Exit,
    Echo,
    Unknown,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "exit" => Command::Exit,
            "echo" => Command::Echo,
            _ => Command::Unknown,
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let (command, args) = input.trim().split_once(' ').unwrap_or(("", ""));
        match Command::from(command) {
            Command::Exit => process::exit(0),
            Command::Echo => println!("{}", args),
            Command::Unknown => println!("{}: command not found", input.trim()),
        }
    }
}
