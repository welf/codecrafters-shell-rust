#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

enum Command {
    Exit,
    Echo,
    Type,
    Unknown(String),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "exit" => Command::Exit,
            "echo" => Command::Echo,
            "type" => Command::Type,
            _ => Command::Unknown(s.to_owned()),
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

        let mut parts = input.trim().split_whitespace();

        match Command::from(parts.next().unwrap_or_default()) {
            Command::Exit => process::exit(0),
            Command::Echo => println!("{}", parts.collect::<Vec<&str>>().join(" ")),
            Command::Type => {
                let next_arg = parts.next().unwrap_or_default();
                match Command::from(next_arg) {
                    Command::Unknown(command) => check_command(command),
                    _ => {
                        println!("{} is a shell builtin", next_arg)
                    }
                }
            }
            Command::Unknown(command) => println!("{}: command not found", command),
        }
    }
}

fn check_command(command: String) {
    let path_variable = std::env::var("PATH").unwrap();
    let paths = path_variable.split(':');
    let mut path = paths.filter(|path| {
        let full_command_path = format!("{}/{}", path, command);
        std::path::Path::new(&full_command_path).exists()
    });
    if let Some(p) = path.next() {
        let path = format!("{}/{}", p, command);
        println!("{command} is {path}");
    } else {
        println!("{command} not found");
    }
}
