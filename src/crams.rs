use crate::config::CramsConfig;
use fs_err::File;
use std::io::{BufReader, BufRead};
use crate::error::Error;
use crate::nephrotic;

const HEADER_FOR_TERRA: &str = "entity:cram_sample_id\tpath";

pub(crate) fn process_cram_list(config: CramsConfig) -> Result<(), Error>{
    let file = File::open(config.input)?;
    let reader = BufReader::new(file);
    println!("{}", HEADER_FOR_TERRA);
    for line in reader.lines() {
        let line = line?;
        let path =
            line.strip_prefix(nephrotic::GCS_PREFIX)
                .ok_or_else(|| { Error::from(
                    format!("Line {} does not start with {}.", line, nephrotic::GCS_PREFIX)
                )})?;
        let id =
            path.split('/').next()
                .ok_or_else(|| Error::from(
                    format!("Path {} is not formatted properly.", path)
                ))?;
        println!("{}\t{}", id, line);
    }
    Ok(())
}