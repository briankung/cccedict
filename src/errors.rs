use std::{error, fmt};

pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

#[derive(Debug, Clone)]
pub struct CedictEntryError;

impl fmt::Display for CedictEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid cedict entry input")
    }
}

impl error::Error for CedictEntryError {}
