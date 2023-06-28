use std::io::{BufRead, Error as IoError};
use serde_json::{Map, Value};
use crate::error::Error;

pub(crate) struct JsonReader {
    lines: Box<dyn Iterator<Item=Result<String, IoError>>>,
}

impl JsonReader {
    pub(crate) fn from_reader<R: BufRead + 'static>(reader: R) -> JsonReader {
        let lines = Box::new(reader.lines());
        JsonReader { lines }
    }
}

impl Iterator for JsonReader {
    type Item = Result<Map<String, Value>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            None => { None }
            Some(Err(io_error)) => { Some(Err(Error::from(io_error))) }
            Some(Ok(line)) => {
                match serde_json::from_str::<Value>(&line) {
                    Ok(value) => {
                        match value {
                            Value::Null => {
                                Some(Err(
                                    Error::from("Expected JSON object, but got null.")
                                ))
                            }
                            Value::Bool(_) => {
                                Some(Err(
                                    Error::from("Expected JSON object, but got boolean.")
                                ))
                            }
                            Value::Number(_) => {
                                Some(Err(
                                    Error::from("Expected JSON object, but got a number.")
                                ))
                            }
                            Value::String(_) => {
                                Some(Err(
                                    Error::from("Expected JSON object, but got a string.")
                                ))
                            }
                            Value::Array(_) => {
                                Some(Err(
                                    Error::from("Expected JSON object, but got an array.")
                                ))
                            }
                            Value::Object(map) => { Some(Ok(map)) }
                        }
                    }
                    Err(error) => { Some(Err(Error::from(error))) }
                }
            }
        }
    }
}