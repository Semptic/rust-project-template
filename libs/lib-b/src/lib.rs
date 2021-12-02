use log::{debug, info};

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum {{crate_name | pascal_case}}Error {
    #[error("Unknown {{crate_name}} Error")]
    Unknown,
}

pub type ResultT<T> = Result<T, {{crate_name | pascal_case}}Error>;

pub fn add(number_1: i64, number_2: i64) -> ResultT<i64> {
    info!("I'm going to add {} and {}", number_1, number_2);
    Ok(number_1 + number_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            add(1, 10),
            Ok(11)
        );
    }
}
