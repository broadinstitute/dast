use crate::config::GroupConfig;
use crate::error::Error;
use fs_err::File;
use std::io::{BufReader, BufRead};
use crate::col_names::ColNames;
use crate::lang::runtime::{map_err_run, RunError, RunResult};
use crate::Value;

pub(crate) fn group_old(config: GroupConfig) -> Result<(), Error> {
    group(&config.input, &config.key_col, &config.value_col)?;
    Ok(())
}

pub(crate) fn group(input: &str, key_col: &str, value_col: &str) -> RunResult {
    let file = map_err_run(File::open(input), input)?;
    let reader = BufReader::new(file);
    let col_names = ColNames::new();
    let i_key = map_err_run(col_names.index_for(key_col), key_col)?;
    let i_value = map_err_run(col_names.index_for(value_col), value_col)?;
    let mut lines = reader.lines();
    if let Some(line) = lines.next() {
        let line = map_err_run(line, input)?;
        let (key_first, value_first) =
            parse_line(key_col, value_col, i_key, i_value, &line)?;
        let mut key_current = String::from(key_first);
        let mut values = vec![String::from(value_first)];
        for line in lines {
            let line = map_err_run(line, input)?;
            let (key_next, value_next) =
                parse_line(key_col, value_col, i_key, i_value, &line)?;
            if key_current.as_str() == key_next {
                values.push(String::from(value_next));
            } else {
                flush(&key_current, &mut values);
                key_current = String::from(key_next);
                values.push(String::from(value_next));
            }
        }
        flush(&key_current, &mut values);
    }
    Ok(Value::Unit)
}

fn get_or_error<'a>(fields: &[&'a str], index: usize, col_name: &str) -> Result<&'a str, RunError> {
    let value =
        *fields.get(index).ok_or_else(|| {
            RunError::from(format!("Column {} expected at {}, but have only {} fields",
                                   col_name, index, fields.len()))
        })?;
    Ok(value)
}

fn parse_line<'a>(key_col: &str, value_col: &str, i_key: usize, i_value: usize, line: &'a str)
                  -> Result<(&'a str, &'a str), RunError> {
    let fields: Vec<&str> = line.split('\t').collect();
    let key = get_or_error(&fields, i_key, key_col)?;
    let value = get_or_error(&fields, i_value, value_col)?;
    Ok((key, value))
}

fn flush(key: &str, values: &mut Vec<String>) {
    println!("{}\t{}", key, drain_and_join_values(values));
}

fn drain_and_join_values(values: &mut Vec<String>) -> String {
    let mut string = String::new();
    string.push('[');
    let mut value_iter = values.drain(..);
    if let Some(value) = value_iter.next() {
        string.push('"');
        string.push_str(&value);
        string.push('"');
        for value in value_iter {
            string.push_str(", \"");
            string.push_str(&value);
            string.push('"');
        }
    }
    string.push(']');
    string
}