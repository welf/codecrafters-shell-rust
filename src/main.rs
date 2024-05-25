use std::{
    fmt::Display,
    io::{self, Write},
    process,
};

/// Command name
type Cmd<'a> = &'a str;
/// Command path
type CommandPath = String;
/// Command arguments
type Args<'a> = Vec<&'a str>;

#[derive(Debug)]
enum Command<'a> {
    Exit,                                            // Built-in command
    Echo(Args<'a>),                                  // Built-in command
    Type(Args<'a>),                                  // Built-in command
    ExternalCommand(Cmd<'a>, CommandPath, Args<'a>), // External command (executable)
    Unknown(Cmd<'a>),                                // Unknown command
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(s: &'a str) -> Self {
        // Split the input string into command and arguments
        let mut parts = s.split_whitespace();
        // Get the command
        let command = parts.next().unwrap_or_default();
        // Get the arguments
        let args = parts.collect();

        match command {
            "exit" => Command::Exit,
            "echo" => Command::Echo(args),
            "type" => Command::Type(args),
            // Check if the command is a known external command
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
                // Join arguments into a single string, separated by spaces
                let command_string = args.join(" ");
                // Parse the string to get the command
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
                // Execute the command and get its output
                let output = process::Command::new(command_path).args(args).output();
                match output {
                    Ok(output) => {
                        if output.status.success() {
                            // Write the output to the standard output
                            io::stdout().write_all(&output.stdout).unwrap();
                        } else {
                            // Write the error code to the standard error
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
    // We can get the executable name or the full path to the executable, so we need to check both
    let cmd = command.split('/').last().unwrap_or_default();
    let path_variable = std::env::var("PATH");
    match path_variable {
        Ok(paths) => {
            let paths = paths.split(':');

            match paths
                .filter(|path| {
                    // Check if the executable is in one of the PATH directories
                    let full_command_path = format!("{}/{}", path, command);
                    std::path::Path::new(&full_command_path).exists()
                })
                .next()
            {
                // If the executable is found in one of PATH directories, return its full path
                Some(path) => Some(format!("{}/{}", path, cmd)),
                None => {
                    // If we got the full path to the executable, return it
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
