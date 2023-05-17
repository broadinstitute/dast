use std::io::{BufRead, Error as IoError};
use crate::error::Error;

pub(crate) struct TsvReader {
    header: Vec<String>,
    lines: Box<dyn Iterator<Item=Result<String, IoError>>>,
}

impl TsvReader {
    pub(crate) fn from_reader<R: BufRead + 'static>(reader: R) -> Result<TsvReader, Error> {
        let mut lines = Box::new(reader.lines());
        let header =
            lines.next().ok_or_else(|| { Error::from("No header line") })??
                .split('\t').map(|s| s.to_string()).collect::<Vec<String>>();
        Ok(TsvReader { header, lines })
    }
}

impl Iterator for TsvReader {
    type Item = Result<Vec<String>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            None => { None }
            Some(Err(io_error)) => { Some(Err(Error::from(io_error))) }
            Some(Ok(string)) => {
                let strings =
                    string.split('\t').map(|s| s.to_string()).collect::<Vec<String>>();
                Some(Ok(strings))
            }
        }
    }
}