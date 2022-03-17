use crate::config::Config;
use crate::error::Error;

mod config;
mod error;
mod crams;
mod fastqs;
mod nephrotic;

pub fn run() -> Result<(), Error> {
    let config = Config::new()?;
    match config {
        Config::CramList(cram_list_config) => {
            crams::process_cram_list(cram_list_config)?
        }
        Config::FastqList(fastq_list_config) => {
            fastqs::process_fastq_list(fastq_list_config)?
        }
    }
    Ok(())
}
