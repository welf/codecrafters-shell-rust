<!--toc:start-->
- [Challenge Status](#challenge-status)
- [Running the Shell](#running-the-shell)
<!--toc:end-->

[![progress-banner](https://backend.codecrafters.io/progress/shell/4ae060a4-3f40-417e-81a3-41b43fa10ab6)](https://app.codecrafters.io/users/welf?r=2qF)

This is a repo for Rust solutions to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

# Challenge Status

The entry point for this `shell` implementation is in `src/main.rs`. This simple `shell`
implementation includes:
- [x] REPL implementation (run `./your_shell.sh` to start the REPL)
- [x] handling unknown commands
- [x] `echo`, `exit`, and `type` built-in commands
- [x] printing the full path to external executables if they are found in the PATH (run
  `type mkdir` in the REPL as an example)
- [x] running external executables with arguments (run `ls -l -s` in the REPL as an
  example)
- [x] implementing the `pwd` built-in command
- [x] implementing the `cd` built-in command

# Running the Shell

1. Ensure you have `cargo (1.70)` installed locally
1. Run `./your_shell.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Execute commands in the REPL that appears. You can run any command that you
   would normally run in a shell.
