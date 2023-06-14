use crate::data::tsv::TsvReader;
use crate::error::Error;
use crate::lang::value::Value;

fn parse_numbers(values: &[String], is: &[usize]) -> Result<Vec<f64>, Error> {
    let mut numbers: Vec<f64> = Vec::new();
    for i in is {
        let number = values.get(*i).ok_or_else(|| {
            Error::from(format!("Index {} is out of range 0..{}", i, values.len() - 1))
        })?.parse::<f64>()?;
        numbers.push(number);
    }
    Ok(numbers)
}

pub(crate) fn covs(file: &str, cols: &[String]) -> Result<Value, Error> {
    println!("File: {}", file);
    println!("Cols: {}", cols.join(", "));
    let mut reader = TsvReader::from_file(file)?;
    let is = reader.cols_to_is(cols)?;
    println!("i_cols: {}", is.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "));
    Ok(Value::Unit)
}