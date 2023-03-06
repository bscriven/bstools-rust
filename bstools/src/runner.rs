use std::path;

use crate::filesystem;

// TODO: Add runner
pub struct RunnerCommand {
    pub command_path: path::PathBuf,
    pub args: Vec<String>
}

#[derive(Clone)]
pub struct Runner {
    pub name: String,
    pub path: path::PathBuf,
    pub command_prefix: String,
    pub command_suffix: String
}

// TODO: Create run_command

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

pub fn get_root_options(runners: Vec<Runner>) -> Vec<filesystem::DirectoryEntry> {
    let mut results: Vec<filesystem::DirectoryEntry> = Vec::new();

    for runner in runners {
        let entries = get_root_options_from_path(runner.path);

        for entry in entries {
            results.push(entry);
        }
    }

    results.sort_by(|a, b| a.name.cmp(&b.name));

    return results;
}

fn get_root_options_from_path(root_path: path::PathBuf) -> Vec<filesystem::DirectoryEntry> {
    return filesystem::get_directory_entries(root_path);
}

pub fn get_options(runners: Vec<Runner>, args: Vec<String>) -> Option<Vec<filesystem::DirectoryEntry>> {
    let mut results: Vec<filesystem::DirectoryEntry> = Vec::new();
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

fn get_options_from_path(root_path: path::PathBuf, args: Vec<String>) -> Option<Vec<filesystem::DirectoryEntry>> {
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

    for runner in runners {
        let command_option = get_command_from_path(runner.path, args.clone());

        if command_option.is_some() && !found_command {
            command = Some(command_option.unwrap());
            found_command = true;
        }
        else if command_option.is_some() && found_command {
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

fn get_command_from_path(root_path: path::PathBuf, args: Vec<String>) -> Option<RunnerCommand> {
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
        let command = RunnerCommand {
            command_path: command_path,
            args: command_args
        };

        return Some(command);
    }

    return None;
}
