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

enum ShellCommand<'a> {
    Cd(Option<&'a str>), // Built-in command
    Exit,                // Built-in command
    Echo(Args<'a>),      // Built-in command
    Pwd,                 // Built-in command
    Type(Args<'a>),      // Built-in command
    Executable {
        command: Cmd<'a>,
        command_path: CommandPath,
        args: Args<'a>,
    }, // External command (executable)
    Unknown(Cmd<'a>),    // Unknown command
}

impl<'a> From<&'a str> for ShellCommand<'a> {
    fn from(s: &'a str) -> Self {
        // Split the input string into command and arguments
        let mut parts = s.split_whitespace();
        // Get the command
        let command = parts.next().unwrap_or_default();
        // Get the arguments
        let args: Vec<&str> = parts.collect();

        match command {
            "cd" => ShellCommand::Cd(args.first().cloned()),
            "exit" => ShellCommand::Exit,
            "echo" => ShellCommand::Echo(args),
            "pwd" => ShellCommand::Pwd,
            "type" => ShellCommand::Type(args),
            // Check if the command is a known external command
            _ => match find_command_path(command) {
                Some(command_path) => ShellCommand::Executable {
                    command,
                    command_path,
                    args,
                },
                None => ShellCommand::Unknown(command),
            },
        }
    }
}

impl<'a> Display for ShellCommand<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellCommand::Cd(_) => write!(f, "cd"),
            ShellCommand::Exit => write!(f, "exit"),
            ShellCommand::Echo(_) => write!(f, "echo"),
            ShellCommand::Pwd => write!(f, "pwd"),
            ShellCommand::Type(_) => write!(f, "type"),
            ShellCommand::Executable { command, .. } => write!(f, "{}", command),
            ShellCommand::Unknown(command) => write!(f, "{}", command),
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        match ShellCommand::from(input.trim()) {
            ShellCommand::Cd(path) => {
                let path = path.unwrap_or("~");

                let path = if path.starts_with('~') {
                    let stripped_path = if path.starts_with("~/") {
                        // We need to strip the slash as well to join the path with the home directory
                        path.strip_prefix("~/").unwrap()
                    } else {
                        // Strip the tilde from the path
                        path.strip_prefix('~').unwrap()
                    };
                    // Expand the tilde to the home directory
                    let home_dir = std::env::var("HOME").expect("Failed to get the home directory");
                    // Join the home directory with the stripped path
                    std::path::Path::new(&home_dir).join(stripped_path)
                } else {
                    std::path::PathBuf::from(path)
                };
                // Change the current working directory
                if let Err(_e) = std::env::set_current_dir(&path) {
                    println!("{}: No such file or directory", path.display());
                }
            }
            ShellCommand::Exit => process::exit(0),
            ShellCommand::Echo(args) => println!("{}", args.join(" ")),
            ShellCommand::Pwd => {
                // Get the current working directory
                let cwd =
                    std::env::current_dir().expect("Failed to get the current working directory");
                println!("{}", cwd.display());
            }
            ShellCommand::Type(args) => {
                // Join arguments into a single string, separated by spaces
                let command_string = args.join(" ");
                // Parse the string to get the command
                let cmd = ShellCommand::from(command_string.as_str());

                match cmd {
                    ShellCommand::Executable {
                        command,
                        command_path,
                        ..
                    } => {
                        println!("{} is {}", command, command_path)
                    }
                    ShellCommand::Unknown(command) => println!("{}: not found", command),
                    _ => {
                        println!("{} is a shell builtin", cmd)
                    }
                }
            }
            ShellCommand::Executable {
                command_path, args, ..
            } => {
                // Execute the command and get its output
                let output = process::Command::new(command_path).args(args).output();
                match output {
                    Ok(output) => {
                        if output.status.success() {
                            // Write the output to the standard output
                            io::stdout()
                                .write_all(&output.stdout)
                                .expect("Failed to write to stdout");
                        } else {
                            // Write the error code to the standard error
                            eprintln!(
                                "Command failed with the status code {}",
                                output
                                    .status
                                    .code()
                                    .expect("Failed to get the output's status code")
                            )
                        }
                    }
                    Err(e) => eprintln!("Error executing command {}", e),
                }
            }
            ShellCommand::Unknown(command) => println!("{}: command not found", command),
        }
    }
}

fn find_command_path(command: &str) -> Option<String> {
    // We can get the executable name with its path in PATH variable or the full path to the executable,
    // so we need to check both cases
    let path_variable = std::env::var("PATH");
    match path_variable {
        Ok(paths) => {
            let mut paths = std::env::split_paths(&paths);

            match paths.find(|path| {
                // Check if the executable is in one of the PATH directories
                let full_command_path = format!("{}/{}", path.display(), command);
                std::path::Path::new(&full_command_path).exists()
            }) {
                // If the executable is found in one of PATH directories, return its full path
                Some(path) => Some(format!("{}/{}", path.display(), command)),
                None => {
                    // If the command itself is the full path to the executable, return it
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
