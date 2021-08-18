use log::{debug, info};

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum {{crate_name | pascal_case}}Error {
    #[error("I will not say 'Hello, {0}.'!")]
    HelloWorldError(String),
    #[error("Unknown {{project-name}} Error")]
    Unknown,
}

pub type ResultT<T> = Result<T, {{crate_name | pascal_case}}Error>;

pub fn hello(name: &str) -> ResultT<()> {
    debug!("Hi, I'm chatty. I want to tell you a great story!");
    if name.to_lowercase() == "world" {
        Err({{crate_name | pascal_case}}Error::HelloWorldError(name.to_string()))
    } else {
        info!("I'm going to greet now.");
        println!("Hello, {}.", name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_error() {
        assert_eq!(
            hello("World"), 
            Err({{crate_name | pascal_case}}Error::HelloWorldError("World".to_string()))
        );
        assert_eq!(
            hello("wOrld"), 
            Err({{crate_name | pascal_case}}Error::HelloWorldError("wOrld".to_string()))
        );
    }

    #[test]
    fn hello_world_success() {
        assert_eq!(hello("Semptic"), Ok(()));
    }
}
