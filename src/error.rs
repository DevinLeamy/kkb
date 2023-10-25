use std::fmt::{self, Display, Formatter};

pub type Result<T> = std::result::Result<T, KKBError>;

#[derive(Debug)]
pub enum KKBError {
    GenError(&'static str),
    Undefined(&'static str),
}

impl Display for KKBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            KKBError::GenError(msg) => write!(f, "Failed to generate an image: {}", msg),
            KKBError::Undefined(msg) => write!(f, "Error: {}", msg),
        }
    }
}
