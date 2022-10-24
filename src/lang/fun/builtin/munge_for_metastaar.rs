use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use jati::trees::types::Type;
use jati::trees::symbols::ArgsFailure;
use crate::data::csv;
use crate::lang::fun::Fun;
use crate::lang::value::Value;
use std::io::Write;
use crate::error::{Error, map_err};
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::Runtime;

pub(crate) struct MungeForMetastaar {}

impl Gen for MungeForMetastaar {
    fn new() -> MungeForMetastaar { MungeForMetastaar {} }
}

fn map_header(header_raw: String) -> String {
    let header = header_raw.to_lowercase();
    if header == "participant id" {
        String::from("id")
    } else if header.starts_with("diabetes") {
        String::from("diabetes")
    } else if header.starts_with("age") {
        String::from("age")
    } else if header.starts_with("weight") {
        String::from("weight")
    } else if header.starts_with("standing height") {
        String::from("height")
    } else if header.starts_with("cancer") {
        String::from("cancer")
    } else {
        header
    }
}

fn warn_value(value: &str) {
    eprintln!("Problematic data value '{}'.", value)
}

fn convert_to_number(value: &str) -> f64 {
    match value.parse::<f64>() {
        Ok(number) => { number }
        Err(_) => {
            let value = value.to_lowercase();
            match value.as_str() {
                "" => {
                    warn_value(&value);
                    0.0
                }
                "do not know" => {
                    warn_value(&value);
                    0.0
                }
                "prefer not to answer" => {
                    warn_value(&value);
                    0.0
                }
                "no" => { 0.0 }
                "yes" => { 1.0 }
                "female" => { 0.0 }
                "male" => { 1.0 }
                _ => {
                    if value.starts_with("yes ") {
                        0.1
                    } else {
                        warn_value(&value);
                        0.0
                    }
                }
            }
        }
    }
}

impl Fun for MungeForMetastaar {
    fn tpe(&self) -> Type { Type::Unit }
    fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure> {
        check_n_args(arg_types, 0)
    }
    fn call(&self, args: Vec<Value>, runtime: &mut Runtime) -> Result<Value, Error> {
        if !args.is_empty() {
            return Err(Error::from("Fun takes no arguments"));
        }
        let env = runtime.env();
        let input_file_name = env.get_arg("i")?;
        let output_file_name = env.get_arg("o")?;
        let reader =
            BufReader::new(
                map_err(File::open(input_file_name), input_file_name)?
            );
        let mut lines = reader.lines();
        let header_line =
            map_err(map_err(lines.next().ok_or_else(|| {
                Error::from("File is empty")
            }), input_file_name)?, input_file_name)?;
        let headers: Vec<String> =
            map_err(csv::parse_line(&header_line), input_file_name)?
                .into_iter().map(map_header).collect();
        let mut writer =
            BufWriter::new(
                map_err(File::create(output_file_name), output_file_name)?
            );
        for header in &headers {
            println!("{}", header)
        }
        map_err(writeln!(writer, "{}", headers.join(",")), output_file_name)?;
        for line in lines {
            let line = map_err(line, input_file_name)?;
            let values = map_err(csv::parse_line(&line), output_file_name)?;
            let mut numbers =
                values.iter().map(|value| { convert_to_number(value) });
            let mut out_line = String::new();
            if let Some(number) = numbers.next() {
                out_line.push_str(&number.to_string());
                for number in numbers {
                    out_line.push(',');
                    out_line.push_str(&number.to_string());
                }
            }
            map_err(writeln!(writer, "{}", out_line), output_file_name)?;
        }
        Ok(Value::Unit)
    }
}