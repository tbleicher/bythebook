use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ResolverError {
    details: String,
}

impl ResolverError {
    pub fn new(msg: &str) -> ResolverError {
        ResolverError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ResolverError {
    fn description(&self) -> &str {
        &self.details
    }
}
