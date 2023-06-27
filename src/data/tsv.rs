use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
use crate::data::csv;
use crate::error::Error;

#[derive(Copy, Clone)]
pub(crate) enum LineParser {
    Tsv,
    Csv,
}

impl LineParser {
    pub(crate) fn parse(name: &str) -> Result<LineParser, Error> {
        match name.trim().to_lowercase().as_str() {
            "tsv" => { Ok(LineParser::Tsv) }
            "csv" => { Ok(LineParser::Csv) }
            _ => Err(Error::from(
                format!("Unknown file format {}. Use 'tsv' or 'csv'", name.trim())
            ))
        }
    }
    fn parse_line(&self, line: &str) -> Result<Vec<String>, Error> {
        match self {
            LineParser::Tsv => { Ok(split_by_tab(line)) }
            LineParser::Csv => { csv::parse_line(line) }
        }
    }
}

fn split_by_tab(line: &str) -> Vec<String> {
    line.split('\t').map(|s| s.to_string()).collect::<Vec<String>>()
}

pub(crate) struct TsvReader {
    line_parser: LineParser,
    pub(crate) header: Vec<String>,
    lines: Box<dyn Iterator<Item=Result<String, IoError>>>,
}

fn error_missing_col(col: &str) -> Error {
    Error::from(format!("Missing column {}.", col))
}

impl TsvReader {
    pub(crate) fn from_reader<R: BufRead + 'static>(reader: R, line_parser: LineParser)
                                                    -> Result<TsvReader, Error> {
        let mut lines = Box::new(reader.lines());
        let header =
            line_parser.parse_line(&lines.next().ok_or_else(|| {
                Error::from("No header line")
            })??)?;
        Ok(TsvReader { line_parser, header, lines })
    }
    pub(crate) fn from_file(file: &str, line_parser: LineParser) -> Result<TsvReader, Error> {
        Self::from_reader(BufReader::new(File::open(file)?), line_parser)
    }
    pub(crate) fn col_to_i(&self, col: &str) -> Result<usize, Error> {
        self.header.iter().position(|s| s.as_str() == col)
            .ok_or_else(|| { error_missing_col(col) })
    }
    pub(crate) fn cols_to_is(&self, cols: &[String]) -> Result<Vec<usize>, Error> {
        let mut i_by_col: BTreeMap<&String, usize> = BTreeMap::new();
        for (i, col) in self.header.iter().enumerate() {
            if cols.contains(col) {
                i_by_col.insert(col, i);
            }
        }
        let mut is: Vec<usize> = Vec::new();
        for col in cols {
            match i_by_col.get(col) {
                None => { return Err(error_missing_col(col)); }
                Some(i) => { is.push(*i) }
            }
        }
        Ok(is)
    }
}

impl Iterator for TsvReader {
    type Item = Result<Vec<String>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            None => { None }
            Some(Err(io_error)) => { Some(Err(Error::from(io_error))) }
            Some(Ok(string)) => {
                Some(self.line_parser.parse_line(&string))
            }
        }
    }
}