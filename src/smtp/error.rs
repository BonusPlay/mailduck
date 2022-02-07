use std::fmt;

#[derive(Debug, Clone)]
pub enum SmtpError {
    InvalidRequest,
}

impl std::error::Error for SmtpError {}

impl fmt::Display for SmtpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmtpError::InvalidRequest => write!(f, "Invalid Request"),
        }
    }
}
