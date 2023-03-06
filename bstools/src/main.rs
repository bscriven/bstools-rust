use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;

use crate::filesystem::DirectoryEntry;
use crate::runner::RunnerCommand;

mod environment;
mod filesystem;
mod runner;

fn main() {
    const HOME_ENVIRONMENT_VARIABLE: &str = "BS_HOME";
    let home_path_option = environment::get_environment_variable(HOME_ENVIRONMENT_VARIABLE);

    match home_path_option {
        None => {
            eprintln!("Mandatory environment variable '{}' does not exist. Set the environment variable and try again.", HOME_ENVIRONMENT_VARIABLE);
            eprintln!("'{}' must contain a directory path. Within the directory, commands and data will be stored.", HOME_ENVIRONMENT_VARIABLE);
            eprintln!("If this is a new installation, an empty directory may be used.");

            process::exit(1);
        }
        Some(home_path_string) => {
            let home_path = Path::new(home_path_string.as_str());
            let data_path = Path::join(home_path, "data");
            let bin_path = Path::join(home_path, "bin");

            fs::create_dir_all(data_path.clone()).expect("Failed to create data directory.");

            let runners = runner::get_runners(home_path.to_path_buf());

            for runner in &runners {
                fs::create_dir_all(runner.path.clone()).expect("Failed to create bin directory.");
            }

            let args: Vec<String> = std::env::args().skip(1).collect();
            let mut runner_options: Vec<DirectoryEntry> = Vec::new();
            let mut command: Option<RunnerCommand> = None;

            if args.len() == 0 {
                runner_options = runner::get_root_options(runners);
            }
            else {
                command = runner::get_command(PathBuf::from(bin_path.clone()), args.clone());

                // TODO: Create get_runner_type

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
                let unwrapped_command = command.unwrap();
                let command_string = format!("{}", unwrapped_command.command_path.as_path().display());
                let process_output = process::Command::new(command_string)
                    .args(&unwrapped_command.args)
                    .spawn()
                    .expect("Failed to launch process.");

                let _ = process_output.wait_with_output();
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

            // TODO: Create bin_runner
        }
    }
}