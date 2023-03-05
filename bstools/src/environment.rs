use std::env;

pub fn get_environment_variable(variable_name: &str) -> Option<String> {
    match env::var(variable_name) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}