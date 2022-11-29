use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RepositoryError {
    details: String,
}

impl RepositoryError {
    pub fn new(msg: &str) -> RepositoryError {
        RepositoryError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for RepositoryError {
    fn description(&self) -> &str {
        &self.details
    }
}
