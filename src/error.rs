use std::fmt::{Display, Formatter};
use std::io;
use std::num::ParseIntError;
use clap::parser::MatchesError;
use jati::error::Error as JatiError;

pub enum Error {
    Tsv(TsvError),
    Jati(JatiError),
    Clap(clap::Error),
    Io(io::Error),
    ParseInt(ParseIntError),
    Matches(MatchesError),
}

pub struct TsvError {
    message: String,
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Tsv(TsvError { message })
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::from(String::from(message))
    }
}

impl From<JatiError> for Error {
    fn from(jati_error: JatiError) -> Self { Error::Jati(jati_error) }
}

impl From<clap::Error> for Error {
    fn from(clap_error: clap::Error) -> Self {
        Error::Clap(clap_error)
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::Io(io_error)
    }
}

impl From<ParseIntError> for Error {
    fn from(parse_int_error: ParseIntError) -> Self { Error::ParseInt(parse_int_error) }
}

impl From<MatchesError> for Error {
    fn from(matches_error: MatchesError) -> Self { Error::Matches(matches_error) }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Tsv(tsv_error) => { writeln!(f, "{}", tsv_error.message) }
            Error::Jati(jati_error) => { writeln!(f, "{}", jati_error) }
            Error::Clap(clap_error) => { writeln!(f, "{}", clap_error) }
            Error::Io(io_error) => { writeln!(f, "{}", io_error) }
            Error::ParseInt(parse_int_error) => { writeln!(f, "{}", parse_int_error) }
            Error::Matches(matches_error) => { writeln!(f, "{}", matches_error) }
        }
    }
}

