use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::rc::Rc;
use jati::trees::types::Type;
use crate::data::csv;
use crate::Error;
use crate::nitro::env::Env;
use crate::nitro::fun::{Fun, FunImpl};
use crate::nitro::value::Value;
use std::io::Write;

pub(crate) struct MungeForMetastaar {}

pub(crate) const NAME: &str = "munge_for_metastaar";

impl MungeForMetastaar {
    pub(crate) fn new_fun() -> Fun {
        let fun_impl = Rc::new(MungeForMetastaar {});
        let tpe = Type::Unit;
        Fun { fun_impl, tpe }
    }
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
                "" => { warn_value(&value); 0.0 }
                "do not know" => { warn_value(&value); 0.0 }
                "prefer not to answer" => { warn_value(&value); 0.0 }
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

impl FunImpl for MungeForMetastaar {
    fn call(&self, args: Vec<Value>, env: &Env) -> Result<Value, Error> {
        if !args.is_empty() {
            return Err(Error::from(format!("{} takes no parameters.", NAME)));
        }
        let input_file_name = env.get_arg("i")?;
        let output_file_name = env.get_arg("o")?;
        let reader = BufReader::new(File::open(input_file_name)?);
        let mut lines = reader.lines();
        let header_line =
            lines.next().ok_or_else(|| {
                Error::from(format!("Input file {} is empty", input_file_name))
            })??;
        let headers: Vec<String> =
            csv::parse_line(&header_line)?.into_iter().map(map_header).collect();
        let mut writer = BufWriter::new(File::create(output_file_name)?);
        for header in &headers {
            println!("{}", header)
        }
        writeln!(writer, "{}", headers.join(","))?;
        for line in lines {
            let values = csv::parse_line(&line?)?;
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
            writeln!(writer, "{}", out_line)?;
        }
        Ok(Value::Unit)
    }
}