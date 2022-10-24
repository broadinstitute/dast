use std::fmt::{Display, Formatter};
use std::io;
use std::num::ParseIntError;
use jati::error::Error as JatiError;

#[derive(Debug, Clone)]
pub struct Error {
    contexts: Vec<String>,
    message: String,
}

impl Error {
    pub(crate) fn new(message: String) -> Error {
        let contexts: Vec<String> = Vec::new();
        Error { contexts, message }
    }
    pub(crate) fn add_context(self, context: String) -> Error {
        let Error { mut contexts, message } = self;
        contexts.push(context);
        Error { contexts, message }
    }
    pub(crate) fn add_str(self, context: &str) -> Error {
        self.add_context(String::from(context))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for context in self.contexts.iter().rev() {
            write!(f, "{}: ", context)?;
        }
        writeln!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(message: String) -> Self { Error::new(message) }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self { Error::new(String::from(message)) }
}

impl From<JatiError> for Error {
    fn from(jati_error: JatiError) -> Self {
        Error::from(jati_error.to_string()).add_str("Jati")
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::from(io_error.to_string()).add_str("IO")
    }
}

impl From<ParseIntError> for Error {
    fn from(parse_int_error: ParseIntError) -> Self {
        Error::from(parse_int_error.to_string()).add_str("ParseIntError")
    }
}

pub(crate) fn map_err<T, E: std::error::Error>(result: Result<T, E>, name: &str)
                                                   -> Result<T, Error> {
    result.map_err(|error| {
        Error::from(error.to_string()).add_context(name.to_string())
    })
}

