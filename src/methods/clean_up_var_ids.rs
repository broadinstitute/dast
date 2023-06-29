use std::io::{BufReader, BufWriter};
use crate::data::io::line_parser::LineParser;
use crate::data::io::tsv::{TsvReader, TsvWriter};
use crate::data::var_id::VarId;
use crate::error::Error;
use crate::lang::value::Value;
use crate::methods::util::io::{file_or_stdin, file_or_stdout};

pub(crate) fn clean_up_var_ids(input: Option<&str>, output: Option<&str>, line_parser: LineParser)
    -> Result<Value, Error> {
    let reader =
        TsvReader::new(BufReader::new(file_or_stdin(input)?), line_parser)?;
    let mut writer =
        TsvWriter::new(BufWriter::new(file_or_stdout(output)?), &reader.header)?;
    for row in reader {
        let row = row?;
        let cleaned_row =
            row.into_iter().map(clean_up_if_var_id).collect::<Vec<String>>();
        writer.write(&cleaned_row)?;
    }
    Ok(Value::Unit)
}

fn clean_up_if_var_id(string: String) -> String {
    match VarId::parse(&string) {
        Ok(var_id) => { var_id.to_string() }
        Err(_) => { string }
    }
}