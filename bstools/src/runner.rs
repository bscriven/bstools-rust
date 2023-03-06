use std::path;

use crate::filesystem;

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

pub fn get_runners(home_path: path::PathBuf) -> Vec<Runner> {
    let mut runners: Vec<Runner> = Vec::new();

    runners.push(Runner {
        name: "bin".to_string(),
        path: path::Path::join(home_path.as_path(), "bin"),
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
