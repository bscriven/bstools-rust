
use std::env;
use std::error::Error;
use std::path::Path;

use crate::configuration;
use crate::constants;
use crate::filesystem;
use crate::runner;

/// Gets an environment variable from the OS.
pub fn get_environment_variable(variable_name: &str) -> Option<String> {
    match env::var(variable_name) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

/// Verifies that the BS_HOME environment variable is set and that all required subdirectories are created within BS_HOME.
/// 
/// The directory path for BS_HOME is returned if there are no errors.
pub fn verify_environment_configured_and_get_home_path() -> Result<String, Box<dyn Error>> {
    let home_path_option = configuration::get_environment_variable(constants::ENVIRONMENT_HOME);

    match home_path_option {
        None => {
            eprintln!("Mandatory environment variable '{}' does not exist. Set the environment variable and try again.", constants::ENVIRONMENT_HOME);
            eprintln!("'{}' must contain a directory path. Within the directory, commands and data will be stored.", constants::ENVIRONMENT_HOME);
            eprintln!("If this is a new installation, an empty directory may be used.");

            return Err(Box::from("Mandatory environment variable does not exist."));
        }
        Some(home_path_string) => {
            let home_path = Path::new(home_path_string.as_str());
            let data_path = Path::join(home_path, "data");

            filesystem::create_directories(data_path.clone());

            let runners = runner::get_runners(home_path.to_path_buf());

            for runner in &runners {
                filesystem::create_directories(runner.path.clone());
            }

            return Ok(home_path_string);
        }
    }
}