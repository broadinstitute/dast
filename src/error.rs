use std::fmt::{Display, Formatter};
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use jati::error::Error as JatiError;
use serde_json::Error as SerdeJsonError;

#[derive(Debug, Clone)]
pub struct Error {
    messages: Vec<String>,
}

impl Error {
    pub(crate) fn new(message: String) -> Error {
        let messages: Vec<String> = vec!(message);
        Error { messages }
    }
    pub(crate) fn add_context(self, context: String) -> Error {
        let Error { mut messages } = self;
        messages.push(context);
        Error { messages }
    }
    pub(crate) fn add_str(self, context: &str) -> Error {
        self.add_context(String::from(context))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut message_iter = self.messages.iter().rev();
        if let Some(message) = message_iter.next() {
            let mut line = String::from(message);
            for message in message_iter {
                line.push_str(": ");
                if message.contains('\n') || message.contains('\r') || message.len() > 77 {
                    writeln!(f, "{}", line)?;
                    write!(f, "{}", message)?;
                    line = String::new();
                } else if line.len() + message.len() > 77 {
                    writeln!(f, "{}", line)?;
                    line = String::from(message);
                } else {
                    line.push_str(message)
                }
            }
            if !line.is_empty() {
                write!(f, "{}", line)?;
            }
        }
        Ok(())
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

impl From<ParseFloatError> for Error {
    fn from(parse_float_error: ParseFloatError) -> Self {
        Error::from(parse_float_error.to_string()).add_str("ParseFloatError")
    }
}

impl From<SerdeJsonError> for Error {
    fn from(serde_json_error: SerdeJsonError) -> Self {
        Error::from(serde_json_error.to_string()).add_str("Serde-JSON")
    }
}

pub(crate) fn map_err<T, E: std::error::Error>(result: Result<T, E>, name: &str)
                                               -> Result<T, Error> {
    result.map_err(|error| {
        Error::from(error.to_string()).add_context(name.to_string())
    })
}

