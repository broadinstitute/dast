use std::fs::File;
use std::io::{Read, stdin, stdout, Write};
use crate::error::Error;

pub(crate) fn file_or_stdin(file: Option<&str>) -> Result<Box<dyn Read>, Error> {
    match file {
        None => { Ok(Box::new(stdin())) }
        Some(file) => { Ok(Box::new(File::open(file)?)) }
    }
}

pub(crate) fn file_or_stdout(file: Option<&str>) -> Result<Box<dyn Write>, Error> {
    match file {
        None => { Ok(Box::new(stdout()))}
        Some(file) => { Ok(Box::new(File::create(file)?))}
    }
}
