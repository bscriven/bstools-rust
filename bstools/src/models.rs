use std::ffi;
use std::path;

#[derive(Clone)]
pub struct CommandDetails {
    pub command_path: path::PathBuf,
    pub args: Vec<String>,
}

pub struct FileSystemEntry {
    pub name: ffi::OsString,
    pub path: path::PathBuf,
    pub is_directory: bool
}

#[derive(Clone)]
pub struct Runner {
    pub name: String,
    pub path: path::PathBuf,
    pub command_prefix: String,
    pub command_suffix: String
}

#[derive(Clone)]
pub struct RunnerCommand {
    pub command_path: path::PathBuf,
    pub args: Vec<String>,
    pub runner: Runner
}