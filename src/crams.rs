use crate::config::CramListConfig;
use fs_err::File;
use std::io::{BufReader, BufRead};
use crate::error::Error;

const PREFIX: &str = "gs://fc-016ff50b-080a-4838-95cd-801061ce2464/";
const HEADER_FOR_TERRA: &str = "entity:cram_sample_id\tpath";

pub(crate) fn process_cram_list(config: CramListConfig) -> Result<(), Error>{
    let file = File::open(config.input)?;
    let reader = BufReader::new(file);
    println!("{}", HEADER_FOR_TERRA);
    for line in reader.lines() {
        let line = line?;
        let path =
            line.strip_prefix(PREFIX)
                .ok_or_else(|| { Error::from(
                    format!("Line {} does not start with {}.", line, PREFIX)
                )})?;
        let id =
            path.split("/").next()
                .ok_or_else(|| Error::from(
                    format!("Path {} is not formatted properly.", path)
                ))?;
        println!("{}\t{}", id, path);
    }
    Ok(())
}