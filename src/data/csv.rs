use std::mem;
use crate::Error;

enum ParseState {
    BeforeValue,
    UnquotedValue,
    QuotedValue,
    AfterClosingQuote,
}

pub(crate) fn parse_line(line: &str) -> Result<Vec<String>, Error> {
    let mut values: Vec<String> = Vec::new();
    let mut value = String::new();
    let mut state = ParseState::BeforeValue;
    for char in line.chars() {
        match state {
            ParseState::BeforeValue => {
                match char {
                    '"' => {
                        state = ParseState::QuotedValue;
                    }
                    ',' => {
                        push_value(&mut values, &mut value, &mut state);
                    }
                    _ => {
                        state = ParseState::UnquotedValue;
                        value.push(char);
                    }
                }
            }
            ParseState::UnquotedValue => {
                match char {
                    ',' => { push_value(&mut values, &mut value, &mut state); }
                    _ => { value.push(char) }
                }
            }
            ParseState::QuotedValue => {
                match char {
                    '"' => { state = ParseState::AfterClosingQuote; }
                    _ => { value.push(char); }
                }
            }
            ParseState::AfterClosingQuote => {
                match char {
                    '"' => {
                        value.push('"');
                        state = ParseState::QuotedValue;
                    }
                    ',' => {
                        push_value(&mut values, &mut value, &mut state);
                    }
                    _ => {
                        Err(Error::from(format!("After a closing quote, only comma, \
                        opening quote or line end are allowed, but got '{}'", char)))?
                    }
                }
            }
        }
    }
    values.push(value);
    Ok(values)
}

fn push_value(values: &mut Vec<String>, value: &mut String, state: &mut ParseState) {
    let completed_value = mem::take(value);
    values.push(completed_value);
    *state = ParseState::BeforeValue;
}