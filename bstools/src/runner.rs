use std::fs;
use std::path;
use std::process;

use crate::filesystem;
use crate::environment;

#[derive(Clone)]
pub struct RunnerCommand {
    pub command_path: path::PathBuf,
    pub args: Vec<String>,
    pub runner: Runner
}

#[derive(Clone)]
pub struct CommandDetails {
    pub command_path: path::PathBuf,
    pub args: Vec<String>,
}

#[derive(Clone)]
pub struct Runner {
    pub name: String,
    pub path: path::PathBuf,
    pub command_prefix: String,
    pub command_suffix: String
}

const RUNNER_BIN: &str = "bin";
const RUNNER_PYTHON: &str = "python";
const RUNNER_COMMAND: &str = "commands";

const ENVIRONMENT_PYTHON: &str = "BS_PYTHON";

pub fn get_runners(home_path: path::PathBuf) -> Vec<Runner> {
    let mut runners: Vec<Runner> = Vec::new();

    runners.push(Runner {
        name: RUNNER_BIN.to_string(),
        path: path::Path::join(home_path.as_path(), "bin"),
        command_prefix: "".to_string(),
        command_suffix: "".to_string()
    });

    runners.push(Runner {
        name: RUNNER_PYTHON.to_string(),
        path: path::Path::join(home_path.as_path(), "python"),
        command_prefix: "".to_string(),
        command_suffix: "".to_string()
    });

    runners.push(Runner {
        name: RUNNER_COMMAND.to_string(),
        path: path::Path::join(home_path.as_path(), "commands"),
        command_prefix: "".to_string(),
        command_suffix: "".to_string()
    });

    return runners;
}

pub fn get_root_options(runners: Vec<Runner>) -> Vec<filesystem::FileSystemEntry> {
    let mut results: Vec<filesystem::FileSystemEntry> = Vec::new();

    for runner in runners {
        let entries = get_root_options_from_path(runner.path);

        for entry in entries {
            results.push(entry);
        }
    }

    results.sort_by(|a, b| a.name.cmp(&b.name));

    return results;
}

fn get_root_options_from_path(root_path: path::PathBuf) -> Vec<filesystem::FileSystemEntry> {
    return filesystem::get_directory_entries(root_path);
}

pub fn get_options(runners: Vec<Runner>, args: Vec<String>) -> Option<Vec<filesystem::FileSystemEntry>> {
    let mut results: Vec<filesystem::FileSystemEntry> = Vec::new();
    let mut found_valid_directory = false;

    for runner in runners {
        let entries_option = get_options_from_path(runner.path, args.clone());

        if entries_option.is_some() {
            let entries = entries_option.unwrap();
            found_valid_directory = true;

            for entry in entries {
                results.push(entry);
            }
        }
    }

    if results.len() > 0 || found_valid_directory {
        results.sort_by(|a, b| a.name.cmp(&b.name));
    
        return Some(results);
    }
    else {
        return None;
    }
}

fn get_options_from_path(root_path: path::PathBuf, args: Vec<String>) -> Option<Vec<filesystem::FileSystemEntry>> {
    let mut directory_path = root_path.clone();

    for arg in args {
        directory_path.push(arg);

        let entry = filesystem::get_entry(directory_path.clone());

        if entry.is_none() {
            return None;
        }
    }

    return Some(filesystem::get_directory_entries(directory_path));
}

pub fn get_command(runners: Vec<Runner>, args: Vec<String>) -> Option<RunnerCommand> {
    let mut found_command = false;
    let mut command: Option<RunnerCommand> = None;

    for runner in runners.clone() {
        let command_details_option = get_command_from_path(runner.clone().path, args.clone());

        if command_details_option.is_some() && !found_command {
            let command_details = command_details_option.unwrap();
            found_command = true;

            command = Some(RunnerCommand { 
                command_path: command_details.command_path,
                args: command_details.args, 
                runner: Runner {
                    command_prefix: runner.command_prefix,
                    command_suffix: runner.command_suffix,
                    name: runner.name,
                    path: path::PathBuf::from(runner.path)
                }
            });
        }
        else if command_details_option.is_some() && found_command {
            let mut args_string = "".to_string();

            for arg in args {
                if args_string == "" {
                    args_string = arg;
                }
                else {
                    args_string = format!("{} {}", args_string, arg);
                }
            }

            panic!("More than one command exists for arguments '{}'. Commands must be unique.", args_string);
        }
    }

    return command;
}

fn get_command_from_path(root_path: path::PathBuf, args: Vec<String>) -> Option<CommandDetails> {
    let mut command_path = root_path.clone();
    let mut found_command = false;
    let mut command_args: Vec<String> = Vec::new();

    for arg in args {
        if found_command {
            command_args.push(arg);
        }
        else {
            command_path.push(arg.clone());

            let entry = filesystem::get_entry(command_path.clone());

            if entry.is_none() {
                return None;
            }
            else if entry.is_some() && !entry.unwrap().is_directory {
                found_command = true;
            }
        }
    }

    if found_command {
        let command = CommandDetails {
            command_path: command_path,
            args: command_args
        };

        return Some(command);
    }

    return None;
}

pub fn run_command(runner_command: RunnerCommand) {
    if runner_command.runner.name == RUNNER_PYTHON {
        run_python_command(runner_command);
    }
    else if runner_command.runner.name == RUNNER_COMMAND {
        run_command_command(runner_command);
    }
    else {
        run_bin_command(runner_command);
    }
}

fn run_bin_command(runner_command: RunnerCommand) {   
    let command_string = format!("{}", runner_command.command_path.as_path().display());
    let process_output = process::Command::new(command_string)
        .args(runner_command.args)
        .spawn()
        .expect("Failed to launch process.");

    let _ = process_output.wait_with_output();
}

fn run_python_command(runner_command: RunnerCommand) {
    let python_path = environment::get_environment_variable(ENVIRONMENT_PYTHON);

    if python_path.is_none() {
        eprintln!("Mandatory environment variable '{}' does not exist. Set the environment variable and try again.", ENVIRONMENT_PYTHON);
        eprintln!("'{}' must contain the path to the Python executable to use when executing commands.", ENVIRONMENT_PYTHON);
        panic!("Mandatory environment variable does not exist.");
    }

    let mut args: Vec<String> = Vec::new();

    args.push(runner_command.command_path.as_os_str().to_string_lossy().to_string());

    for arg in runner_command.args {
        args.push(arg);
    }

    let process_output = process::Command::new(python_path.unwrap())
        .args(args)
        .spawn()
        .expect("Failed to launch process.");

    let _ = process_output.wait_with_output();
}

fn run_command_command(runner_command: RunnerCommand) {   
    let command_file_contents = fs::read_to_string(runner_command.clone().command_path)
        .expect("Unable to read the command file.");

    let mut command_string = "".to_string();
    let mut consumed_args_count = 0;

    // Replace all %s tokens in the string that was read from the command file
    for command_part in command_file_contents.split("%s") {
        if command_string == "" {
            command_string = format!("{}", command_part);
        }
        else {
            let arg_option = runner_command.clone().args.into_iter().nth(consumed_args_count);

            if arg_option.is_none() {
                eprintln!("The following command expects an argument that was not provided in order to replace the %s token:");
                eprintln!("{}", command_file_contents);
                panic!("Must provide argument to execute the command.");
            }

            command_string = format!("{}{}{}", command_string, arg_option.unwrap(), command_part);
            consumed_args_count += 1;
        }
    }

    // Extract the command to execute from command_string and populate the arguments
    let mut command_to_execute: String = "".to_string();
    let mut args: Vec<String> = Vec::new();

    for command_part in command_string.split(" ") {
        if command_to_execute == "" {
            command_to_execute = format!("{}", command_part);
        }
        else {
            args.push(command_part.to_string());
        }
    }

    // Populate any remaining arguments that should be appended to the command
    for arg in runner_command.args.into_iter().skip(consumed_args_count) {
        args.push(arg.to_string());
    }

    let process_output = process::Command::new(command_to_execute)
        .args(args)
        .spawn()
        .expect("Failed to launch process.");

    let _ = process_output.wait_with_output();
}