use std::mem;
use crate::error::Error;

enum ParseState {
    BeforeValue,
    UnquotedValue,
    QuotedValue,
    AfterClosingQuote,
}

pub(crate) struct LineParser {
    sep: char,
    enable_quoting: bool,
}

impl LineParser {
    pub(crate) fn new(sep: char, enable_quoting: bool) -> LineParser {
        LineParser { sep, enable_quoting }
    }
    pub(crate) fn new_tsv() -> LineParser { LineParser::new('\t', false)}
    pub(crate) fn new_csv() -> LineParser { LineParser::new(',', true)}
    pub(crate) fn new_semi() -> LineParser { LineParser::new(';', true)}
    pub(crate) fn from_name(name: &str) -> Result<LineParser, Error> {
        match name {
            "tsv" => { Ok(LineParser::new_tsv()) }
            "csv" => { Ok(LineParser::new_csv()) }
            "semi" => { Ok(LineParser::new_semi()) }
            _ => { Err(Error::from(format!("Unknown line format '{}'", name))) }
        }
    }
    pub(crate) fn parse(&self, line: &str) -> Result<Vec<String>, Error> {
        if self.enable_quoting {
            self.parse_quoted_line(line)
        } else {
            Ok(line.split(self.sep).map(|s| s.to_string()).collect::<Vec<String>>())
        }
    }
    pub(crate) fn parse_quoted_line(&self, line: &str) -> Result<Vec<String>, Error> {
        let mut values: Vec<String> = Vec::new();
        let mut value = String::new();
        let mut state = ParseState::BeforeValue;
        for char in line.chars() {
            match state {
                ParseState::BeforeValue => {
                    if char == '"' {
                        state = ParseState::QuotedValue;
                    } else if char == self.sep {
                        push_value(&mut values, &mut value, &mut state);
                    } else {
                        state = ParseState::UnquotedValue;
                        value.push(char);
                    }
                }
                ParseState::UnquotedValue => {
                    if char == self.sep {
                        push_value(&mut values, &mut value, &mut state);
                    } else {
                        value.push(char)
                    }
                }
                ParseState::QuotedValue => {
                    match char {
                        '"' => { state = ParseState::AfterClosingQuote; }
                        _ => { value.push(char); }
                    }
                }
                ParseState::AfterClosingQuote => {
                    if char == '"' {
                        value.push('"');
                        state = ParseState::QuotedValue;
                    } else if char == self.sep {
                        push_value(&mut values, &mut value, &mut state);
                    } else {
                        Err(Error::from(format!("After a closing quote, only {}, \
                        opening quote or line end are allowed, but got '{}'", self.sep, char)))?
                    }
                }
            }
        }
        values.push(value);
        Ok(values)
    }
}

fn push_value(values: &mut Vec<String>, value: &mut String, state: &mut ParseState) {
    let completed_value = mem::take(value);
    values.push(completed_value);
    *state = ParseState::BeforeValue;
}