use crate::config::Config;
use crate::error::Error;

mod config;
mod error;
mod crams;
mod fastqs;
mod nephrotic;
mod fastq_bams;

pub fn run() -> Result<(), Error> {
    let config = Config::new()?;
    match config {
        Config::Crams(cram_list_config) => {
            crams::process_cram_list(cram_list_config)?
        }
        Config::Fastqs(fastq_list_config) => {
            fastqs::process_fastq_list(fastq_list_config)?
        }
        Config::FastqBams(fastq_bam_list_config) => {
            fastq_bams::process_fastq_bam_list(fastq_bam_list_config)?
        }
    }
    Ok(())
}
