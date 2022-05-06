use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: String,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<String> for FetchError {
    fn from(value: String) -> Self {
        Self { err: value }
    }
}