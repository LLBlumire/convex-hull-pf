use std::error::Error as TError;
use std::fmt::Display as TDisplay;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnknownError,
}

impl TDisplay for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(
            f,
            "{}",
            match self.kind {
                ErrorKind::UnknownError => format!("an unknown error occured"),
            }
        )
    }
}

impl TError for Error {
    fn description(&self) -> &str {
        "an error occured while processing a convex hull"
    }
}
