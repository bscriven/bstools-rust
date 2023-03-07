use std::fs;
use std::path;
use std::process;

use crate::constants;
use crate::configuration;
use crate::filesystem;
use crate::models::CommandDetails;
use crate::models::FileSystemEntry;
use crate::models::Runner;
use crate::models::RunnerCommand;

/// Gets a list of all runners that are supported by bstools.
pub fn get_runners(home_path: path::PathBuf) -> Vec<Runner> {
    let mut runners: Vec<Runner> = Vec::new();

    runners.push(Runner {
        name: constants::RUNNER_EXECUTABLE.to_string(),
        path: path::Path::join(home_path.as_path(), constants::RUNNER_EXECUTABLE),
        command_prefix: "".to_string(),
        command_suffix: "".to_string()
    });

    runners.push(Runner {
        name: constants::RUNNER_PYTHON.to_string(),
        path: path::Path::join(home_path.as_path(), constants::RUNNER_PYTHON),
        command_prefix: "".to_string(),
        command_suffix: "".to_string()
    });

    runners.push(Runner {
        name: constants::RUNNER_COMMAND.to_string(),
        path: path::Path::join(home_path.as_path(), constants::RUNNER_COMMAND),
        command_prefix: "".to_string(),
        command_suffix: "".to_string()
    });

    runners.push(Runner {
        name: constants::RUNNER_JAVA.to_string(),
        path: path::Path::join(home_path.as_path(), constants::RUNNER_JAVA),
        command_prefix: "".to_string(),
        command_suffix: "".to_string()
    });

    return runners;
}

/// Gets a list of all options that are available at the root level. Options in all runner directories are searched.
/// 
/// Options may include commands to execute as well as directories (which contain commands to execute).
pub fn get_root_options(runners: Vec<Runner>) -> Vec<FileSystemEntry> {
    let mut results: Vec<FileSystemEntry> = Vec::new();

    for runner in runners {
        let entries = filesystem::get_directory_entries(runner.path);

        for entry in entries {
            results.push(entry);
        }
    }

    results.sort_by(|a, b| a.name.cmp(&b.name));

    return results;
}

/// Gets a list of all options that are available given the specified arguments. Options in all runner directories are searched.
/// 
/// Options may include commands to execute as well as directories (which contain commands to execute).
pub fn get_options(runners: Vec<Runner>, args: Vec<String>) -> Option<Vec<FileSystemEntry>> {
    let mut results: Vec<FileSystemEntry> = Vec::new();
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

/// Gets a list of all options that are available at a specific path.
/// 
/// Specific paths may be /[BS_HOME]/python/example, /[BS_HOME]/executables, etc.
fn get_options_from_path(root_path: path::PathBuf, args: Vec<String>) -> Option<Vec<FileSystemEntry>> {
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

/// Attempts to get a command from one of the runner directories using the provided arguments.
/// 
/// If no command is found, None is returned.
/// 
/// Validation is performed to ensure that the command is unique.
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

/// Attempts to get a command from a specific path using the provided arguments.
/// 
/// Specific paths may be /[BS_HOME]/python/example, /[BS_HOME]/executables, etc.
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

/// Runs a command using the appropriate runner.
pub fn run_command(runner_command: RunnerCommand) {
    if runner_command.runner.name == constants::RUNNER_PYTHON {
        run_python(runner_command);
    }
    else if runner_command.runner.name == constants::RUNNER_COMMAND {
        run_command_alias(runner_command);
    }
    else if runner_command.runner.name == constants::RUNNER_JAVA {
        run_java(runner_command);
    }
    else {
        run_executable(runner_command);
    }
}

/// Runs an executable.
fn run_executable(runner_command: RunnerCommand) {   
    let command_string = format!("{}", runner_command.command_path.as_path().display());
    let process_output = process::Command::new(command_string)
        .args(runner_command.args)
        .spawn()
        .expect("Failed to launch process.");

    let _ = process_output.wait_with_output();
}

/// Runs a Python script.
fn run_python(runner_command: RunnerCommand) {
    let python_path = configuration::get_environment_variable(constants::ENVIRONMENT_PYTHON);

    if python_path.is_none() {
        // Panic if the environment variable with the path to the Python executable is not set.
        eprintln!("Mandatory environment variable '{}' does not exist. Set the environment variable and try again.", constants::ENVIRONMENT_PYTHON);
        eprintln!("'{}' must contain the path to the Python executable to use when executing commands.", constants::ENVIRONMENT_PYTHON);
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

/// Runs a command alias.
fn run_command_alias(runner_command: RunnerCommand) {   
    let command_file_contents = fs::read_to_string(runner_command.clone().command_path)
        .expect("Unable to read the command file.");

    if command_file_contents.contains("\n") {
        panic!("Command file contains more than one line. Only single line commands are supported.");
    }

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
                eprintln!("The following command expects one or more arguments in order to replace the %s token(s):");
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

/// Runs an executable Java JAR.
fn run_java(runner_command: RunnerCommand) {
    let java_path = configuration::get_environment_variable(constants::ENVIRONMENT_JAVA);

    if java_path.is_none() {
        // Panic if the environment variable with the path to the Java executable is not set.
        eprintln!("Mandatory environment variable '{}' does not exist. Set the environment variable and try again.", constants::ENVIRONMENT_JAVA);
        eprintln!("'{}' must contain the path to the Java executable to use when executing commands.", constants::ENVIRONMENT_JAVA);
        panic!("Mandatory environment variable does not exist.");
    }

    let mut args: Vec<String> = Vec::new();

    args.push("-jar".to_string());
    args.push(runner_command.command_path.as_os_str().to_string_lossy().to_string());

    for arg in runner_command.args {
        args.push(arg);
    }

    let process_output = process::Command::new(java_path.unwrap())
        .args(args)
        .spawn()
        .expect("Failed to launch process.");

    let _ = process_output.wait_with_output();
}