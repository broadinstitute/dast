use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::data::tsv::TsvReader;
use crate::error::Error;
use crate::lang::value::Value;

fn read_subset_file(subset_file: &str) -> Result<BTreeSet<String>, Error> {
    let mut subset: BTreeSet<String> = BTreeSet::new();
    for line in BufReader::new(File::open(subset_file)?).lines() {
        subset.insert(line?);
    }
    Ok(subset)
}

pub(crate) fn subset_wilcox(ranks_file: &str, ranks_file_col: &str, subset_file: &str)
                            -> Result<Value, Error> {
    let subset = read_subset_file(subset_file)?;
    let ranks = TsvReader::from_file(ranks_file)?;
    let i_col = ranks.col_to_i(ranks_file_col)?;
    let mut n_subset: usize = 0;
    let mut n_others: usize = 0;
    let mut rank_sum: usize = 0;
    for (i_rank, row) in ranks.enumerate() {
        let row = row?;
        let id =
            row.get(i_col).ok_or_else(|| {
                Error::from(format!("Row {} has no value for {}", i_rank, ranks_file_col))
            })?;
        if subset.contains(id) {
            n_subset += 1;
            rank_sum += i_rank + 1;
        } else {
            n_others += 1;
        }
    }
    println!("n_subset = {}", n_subset);
    println!("n_others = {}", n_others);
    let subset_average_rank = (rank_sum as f64) / (n_subset as f64);
    println!("subset_average_rank = {}", subset_average_rank);
    let u = rank_sum - n_subset * (n_subset + 1) / 2;
    println!("u = {}", u);
    Ok(Value::Unit)
}