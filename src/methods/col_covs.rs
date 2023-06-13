use crate::data::tsv::TsvReader;
use crate::error::Error;
use crate::lang::value::Value;

pub(crate) fn col_covs(file: &str, cols: &[String]) -> Result<Value, Error> {
    println!("File: {}", file);
    println!("Cols: {}", cols.join(", "));
    let mut reader = TsvReader::from_file(file)?;
    let is = reader.cols_to_is(cols)?;
    println!("i_cols: {}", is.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "));
    Ok(Value::Unit)
}