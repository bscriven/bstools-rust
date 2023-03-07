use std::path::Path;
use std::process;

mod constants;
mod configuration;
mod models;
mod filesystem;
mod runner;

use crate::models::FileSystemEntry;
use crate::models::RunnerCommand;

fn main() {
    // Ensure that the environment is configured correctly and get the directory path for BS_HOME
    let environment_verification_result = configuration::verify_environment_configured_and_get_home_path();

    if environment_verification_result.is_err() {
        process::exit(1);
    }

    let home_path_string = environment_verification_result.unwrap();
    let home_path = Path::new(home_path_string.as_str());
    let args: Vec<String> = std::env::args().skip(1).collect();
    let runners = runner::get_runners(home_path.to_path_buf());
    let mut runner_options: Vec<FileSystemEntry> = Vec::new();
    let mut command: Option<RunnerCommand> = None;

    if args.len() == 0 {
        // No arguments were provided, so get the options that are available at the root
        runner_options = runner::get_root_options(runners);
    }
    else {
        // Try to form a command to execute from the arguments that were provided
        command = runner::get_command(runners.clone(), args.clone());

        if command.is_none() {
            // If a command cannot be formed, then get the options that are available using the provided arguments
            let get_options_option = runner::get_options(runners, args.clone());
            
            if get_options_option.is_none() {
                // If no options are available, then the provided arguments are invalid. Display an error and exit.
                eprintln!("The command you entered is not valid. Enter a valid command and try again.");

                process::exit(2);
            }
            else {
                runner_options = get_options_option.unwrap();
            }
        }
    }

    if command.is_some() {
        // If a command is available, then execute it
        runner::run_command(command.unwrap());
    }
    else {
        // If no command is available, then display the options that are available
        if runner_options.len() > 0 {
            println!("Available options:");

            for option in runner_options {
                println!("    {}", option.name.to_str().unwrap());
            }
        }
        else {
            if args.len() > 0 {
                eprintln!("The '{}' directory is empty. Try adding commands to the directory.", args.last().unwrap());
            }
            else {
                eprintln!("{} contains no commands. Try adding commands.", home_path_string);
            }
        }
    }
}