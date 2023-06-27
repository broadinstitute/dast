use crate::data::tsv::{LineParser, TsvReader};
use crate::error::Error;
use crate::lang::value::Value;

fn parse_numbers(values: &[String], is: &[usize]) -> Vec<f64> {
    let mut numbers: Vec<f64> = Vec::new();
    for i in is {
        let number = values[*i].parse::<f64>().unwrap_or(0f64);
        numbers.push(number);
    }
    numbers
}

fn join<T: ToString>(items: &[T], sep: &str) -> String {
    items.iter().map(|item| item.to_string()).collect::<Vec<String>>().join(sep)
}

pub(crate) fn covs(file: &str, cols: &[String]) -> Result<Value, Error> {
    println!("File: {}", file);
    println!("Cols: {}", cols.join(", "));
    let reader = TsvReader::from_file(file, LineParser::Tsv)?;
    let is = reader.cols_to_is(cols)?;
    println!("i_cols: {}", join(&is, ", "));
    let n_cols = is.len();
    let mut x_sums: Vec<f64> = vec![0f64; n_cols];
    let mut xy_sums: Vec<Vec<f64>> = vec![vec![0f64; n_cols]; n_cols];
    let mut count: usize = 0;
    for row in reader {
        let row = row?;
        let values = parse_numbers(&row, &is);
        count += 1;
        for (j, x) in values.iter().enumerate() {
            x_sums[j] += x;
            for (k, y) in values.iter().enumerate() {
                xy_sums[j][k] += x * y;
            }
        }
    }
    if count == 0 {
        println!("File has no data rows")
    } else {
        let means: Vec<f64> = x_sums.iter().map(|sum| sum / (count as f64)).collect();
        let covars_raw: Vec<Vec<f64>> =
            xy_sums.iter().zip(means.iter()).map(|(xys, x_mean)| {
                xys.iter().zip(means.iter()).map(|(xy, y_mean)| {
                    let xy_mean = xy / (count as f64);
                    xy_mean - x_mean * y_mean
                }).collect()
            }).collect();
        let covars: Vec<Vec<f64>> =
            covars_raw.iter().enumerate().map(|(j, covs)| {
                covs.iter().enumerate().map(|(k, cov_raw)| {
                    cov_raw / (covars_raw[j][j] * covars_raw[k][k]).sqrt()
                }).collect()
            }).collect();
        println!("\t{}", cols.join("\t"));
        for (j, covars_row) in covars.iter().enumerate() {
            println!("{}\t{}", cols[j], join(covars_row, "\t"))
        }
    }
    Ok(Value::Unit)
}