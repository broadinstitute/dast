use crate::config::Config;
use crate::error::Error;

mod config;
mod error;
mod crams;
mod fastqs;
mod nephrotic;
mod fastq_bams;
mod group;
mod col_names;
mod ubams;

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
        Config::Group(group_config) => {
            group::group(group_config)?
        }
        Config::Ubams(ubams_config) => {
            ubams::create_ubams_lists(ubams_config)?
        }
    }
    Ok(())
}
