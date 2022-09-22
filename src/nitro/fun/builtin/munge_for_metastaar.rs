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
        println!("Yo! At least got this far!");
        todo!()
    }
}