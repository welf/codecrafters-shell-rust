#[allow(unused_imports)]
use std::{
    fmt::Display,
    io::{self, Write},
    process,
};

#[derive(Debug)]
enum Command<'a> {
    Exit,
    Echo(Vec<&'a str>),
    Type(Vec<&'a str>),
    ExternalCommand(&'a str, String, Vec<&'a str>),
    Unknown(&'a str),
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(s: &'a str) -> Self {
        let mut parts = s.split_whitespace();
        let command = parts.next().unwrap_or_default();
        let args = parts.collect();
        match command {
            "exit" => Command::Exit,
            "echo" => Command::Echo(args),
            "type" => Command::Type(args),
            _ => match check_command(command) {
                Some(command_path) => Command::ExternalCommand(command, command_path, args),
                None => Command::Unknown(command),
            },
        }
    }
}

impl<'a> Display for Command<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Exit => write!(f, "exit"),
            Command::Echo(_) => write!(f, "echo"),
            Command::Type(_) => write!(f, "type"),
            Command::ExternalCommand(command, _, _) => write!(f, "{}", command),
            Command::Unknown(command) => write!(f, "{}", command),
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

        match Command::from(input.trim()) {
            Command::Exit => process::exit(0),
            Command::Echo(args) => println!("{}", args.join(" ")),
            Command::Type(args) => {
                let command_string = args.join(" ");
                let cmd = Command::from(command_string.as_str());
                match cmd {
                    Command::ExternalCommand(command, path, _) => {
                        println!("{} is {}", command, path)
                    }
                    Command::Unknown(command) => println!("{}: not found", command),
                    _ => {
                        println!("{} is a shell builtin", cmd)
                    }
                }
            }
            Command::ExternalCommand(_command, command_path, args) => {
                let output = process::Command::new(command_path).args(args).output();
                match output {
                    Ok(output) => {
                        if output.status.success() {
                            io::stdout().write_all(&output.stdout).unwrap();
                        } else {
                            eprintln!(
                                "Command failed with the status code {}",
                                output.status.code().unwrap()
                            )
                        }
                    }
                    Err(e) => eprintln!("Error executing command {}", e),
                }
            }
            Command::Unknown(command) => println!("{}: command not found", command),
        }
    }
}

fn check_command(command: &str) -> Option<String> {
    let path_variable = std::env::var("PATH");
    match path_variable {
        Ok(paths) => {
            let paths = paths.split(':');

            match paths
                .filter(|path| {
                    let full_command_path = format!("{}/{}", path, command);
                    std::path::Path::new(&full_command_path).exists()
                })
                .next()
            {
                Some(path) => Some(format!("{}/{}", path, command)),
                None => {
                    if std::path::Path::new(command).exists() {
                        Some(command.to_string())
                    } else {
                        None
                    }
                }
            }
        }
        Err(_) => None,
    }
}
