use std::fs::File;
use std::io::{BufWriter, Write};
use crate::data::line_parser::LineParser;
use crate::data::tsv::TsvReader;
use crate::error::Error;
use crate::lang::value::Value;

fn calculate_quotient(row: &[String], i_numerator: usize, i_denominator: usize)
                      -> Result<f64, Error> {
    let num = row[i_numerator].parse::<f64>()?;
    let den = row[i_denominator].parse::<f64>()?;
    Ok(num / den)
}

pub(crate) fn add_quotient(input: &str, output: &str, numerator: &str, denominator: &str,
                           col_name: &str, line_parser: LineParser)
                           -> Result<Value, Error> {
    println!("Input: {}", input);
    println!("Output: {}", output);
    println!("Numerator col: {}", numerator);
    println!("Denominator col: {}", denominator);
    let reader = TsvReader::from_file(input, line_parser)?;
    let i_numerator = reader.col_to_i(numerator)?;
    let i_denominator = reader.col_to_i(denominator)?;
    println!("i_numerator: {}", i_numerator);
    println!("i_denominator: {}", i_denominator);
    let mut writer = BufWriter::new(File::create(output)?);
    writer.write_fmt(format_args!("{}\t{}\n", reader.header.join("\t"), col_name))?;
    for row in reader {
        let row = row?;
        let quot_str =
            calculate_quotient(&row, i_numerator, i_denominator)
                .map(|quot| quot.to_string()).unwrap_or("NA".to_string());
        writer.write_fmt(format_args!("{}\t{}\n", row.join("\t"), quot_str))?;
    }
    Ok(Value::Unit)
}
