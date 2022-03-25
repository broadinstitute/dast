use crate::config::FastqBamsConfig;
use crate::error::Error;
use fs_err::File;
use std::io::{BufReader, BufRead};

pub(crate) fn process_fastq_bam_list(config: FastqBamsConfig)
                                     -> Result<(), Error> {
    let file = File::open(config.input)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let file_name =
            line.split('/')
                .last()
                .ok_or_else(|| { Error::from(format!("No slash in '{}'", line)) })?;
        println!("{}\t{}", file_name, line);
    }
    Ok(())
}
