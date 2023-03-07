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
    let home_path_option = configuration::get_environment_variable(constants::ENVIRONMENT_HOME);

    match home_path_option {
        None => {
            eprintln!("Mandatory environment variable '{}' does not exist. Set the environment variable and try again.", constants::ENVIRONMENT_HOME);
            eprintln!("'{}' must contain a directory path. Within the directory, commands and data will be stored.", constants::ENVIRONMENT_HOME);
            eprintln!("If this is a new installation, an empty directory may be used.");

            process::exit(1);
        }
        Some(home_path_string) => {
            let home_path = Path::new(home_path_string.as_str());
            let data_path = Path::join(home_path, "data");

            filesystem::create_directories(data_path.clone());

            let runners = runner::get_runners(home_path.to_path_buf());

            for runner in &runners {
                filesystem::create_directories(runner.path.clone());
            }

            let args: Vec<String> = std::env::args().skip(1).collect();
            let mut runner_options: Vec<FileSystemEntry> = Vec::new();
            let mut command: Option<RunnerCommand> = None;

            if args.len() == 0 {
                runner_options = runner::get_root_options(runners);
            }
            else {
                command = runner::get_command(runners.clone(), args.clone());

                if command.is_none() {
                    let get_options_option = runner::get_options(runners, args.clone());
                    
                    if get_options_option.is_none() {
                        eprintln!("The command you entered is not valid. Enter a valid command and try again.");

                        process::exit(2);
                    }
                    else {
                        runner_options = get_options_option.unwrap();
                    }
                }
            }

            if command.is_some() {
                runner::run_command(command.unwrap());
            }
            else {
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
    }
}