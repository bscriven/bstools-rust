use std::fs;
use std::path;

use crate::models::FileSystemEntry;

pub fn get_directory_entries(path: path::PathBuf) -> Vec<FileSystemEntry> {
    let path_results = fs::read_dir(path).unwrap();
    let mut entries: Vec<FileSystemEntry> = Vec::new(); 

    for path_result in path_results {
        let path_entry = path_result.unwrap();
        let entry = FileSystemEntry {
            name: path_entry.file_name(),
            path: path_entry.path(),
            is_directory: path_entry.file_type().unwrap().is_dir()
        };

        entries.push(entry);
    }

    return entries;
}

pub fn get_entry(path: path::PathBuf) -> Option<FileSystemEntry> {
    let is_existing = path.as_path().exists();
    
    if is_existing {
        let file_name = path.file_name().unwrap().to_os_string();
        let is_directory = path.is_dir();

        let entry = FileSystemEntry {
            name: file_name.clone(),
            path: path.clone(),
            is_directory: is_directory
        };

        return Some(entry);
    }
    else {
        return None;
    }
}