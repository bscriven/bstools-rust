use std::fs;
use std::path::PathBuf;

use crate::models::FileSystemEntry;

/// Gets all entries from the specified directory path. Entries consist of files and subdirectories.
pub fn get_directory_entries(path: PathBuf) -> Vec<FileSystemEntry> {
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

/// Gets a single entry from the specified path. The entry may be a file or a directory.
pub fn get_entry(path: PathBuf) -> Option<FileSystemEntry> {
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

/// Creates all directories included in the specified path.
pub fn create_directories(path: PathBuf) {
    let cloned_path = path.clone();
    let file_name = cloned_path.file_name().unwrap().to_string_lossy();

    fs::create_dir_all(path).expect(format!("Failed to create {} directory.", file_name).as_str());
}