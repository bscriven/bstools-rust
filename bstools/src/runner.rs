use std::path;

use crate::filesystem;

pub struct Command {
    pub command_path: path::PathBuf,
    pub args: Vec<String>
}

pub fn get_root_options(root_path: path::PathBuf) -> Vec<filesystem::DirectoryEntry> {
    return filesystem::get_directory_entries(root_path);
}

pub fn get_options(root_path: path::PathBuf, args: Vec<String>) -> Option<Vec<filesystem::DirectoryEntry>> {
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

pub fn get_command(root_path: path::PathBuf, args: Vec<String>) -> Option<Command> {
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
        let command = Command {
            command_path: command_path,
            args: command_args
        };

        return Some(command);
    }

    return None;
}
