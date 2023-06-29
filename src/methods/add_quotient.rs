use std::io::{BufReader, BufWriter};
use crate::data::io::line_parser::LineParser;
use crate::data::io::tsv::{TsvReader, TsvWriter};
use crate::error::Error;
use crate::lang::value::Value;
use crate::methods::util::io::{file_or_stdin, file_or_stdout};

fn calculate_quotient(row: &[String], i_numerator: usize, i_denominator: usize)
                      -> Result<f64, Error> {
    let num = row[i_numerator].parse::<f64>()?;
    let den = row[i_denominator].parse::<f64>()?;
    Ok(num / den)
}

pub(crate) fn add_quotient(input: Option<&str>, output: Option<&str>, numerator: &str,
                           denominator: &str, col_name: &str, line_parser: LineParser)
                           -> Result<Value, Error> {
    eprintln!("Input: {}", input.unwrap_or("<STDIN>"));
    eprintln!("Output: {}", output.unwrap_or("<STDOUT>"));
    eprintln!("Numerator col: {}", numerator);
    eprintln!("Denominator col: {}", denominator);
    let reader =
        TsvReader::new(BufReader::new(file_or_stdin(input)?), line_parser)?;
    let i_numerator = reader.col_to_i(numerator)?;
    let i_denominator = reader.col_to_i(denominator)?;
    eprintln!("i_numerator: {}", i_numerator);
    eprintln!("i_denominator: {}", i_denominator);
    let out_header = {
        let mut out_header = reader.header.clone();
        out_header.push(col_name.to_string());
        out_header
    };
    let mut writer =
        TsvWriter::new(BufWriter::new(file_or_stdout(output)?),
                       &out_header)?;
    for row in reader {
        let mut row = row?;
        let quot_str =
            calculate_quotient(&row, i_numerator, i_denominator)
                .map(|quot| quot.to_string()).unwrap_or("NA".to_string());
        row.push(quot_str);
        writer.write(&row)?;
    }
    Ok(Value::Unit)
}
