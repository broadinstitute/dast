use crate::config::UbamsConfig;
use crate::error::Error;
use fs_err::File;
use std::io::{BufReader, BufRead, BufWriter, Write};

fn extract_sample_id(file_name: &str) -> Result<&str, Error> {
    let without_cohort =
        file_name.split("__").last().ok_or_else(|| {
            Error::from(format!("Cannot parse '{}'.", file_name))
        })?;
    if let Some(sample_id) = without_cohort.strip_suffix(".unmapped.bam") {
        Ok(sample_id)
    } else if let Some(sample_id) = without_cohort.strip_suffix(".bam") {
        Ok(sample_id)
    } else {
        Err(Error::from(format!("Cannot parse '{}'.", file_name)))
    }
}

fn write_ubams_list(file_name: &str, content: &str) -> Result<(), Error> {
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", content)?;
    writer.flush()?;
    Ok(())
}

pub(crate) fn create_ubams_lists(config: UbamsConfig) -> Result<(), Error> {
    let input_file = File::open(&config.input)?;
    let reader = BufReader::new(input_file);
    let output_file = File::create(&config.output)?;
    let mut writer = BufWriter::new(output_file);
    writeln!(writer, "entity:sample_id\tubam\tubams_list")?;
    for line in reader.lines() {
        let line = line?;
        let ubam_file_name =
            line.split('/').last().ok_or_else(|| {
                Error::from(format!("Cannot parse line '{}'", line))
            })?;
        let sample_id = extract_sample_id(ubam_file_name)?;
        let ubam_list_file_local = format!("{}{}.ubams", config.target, sample_id);
        let ubam_list_file_remote = format!("{}{}.ubams", config.prefix, sample_id);
        write_ubams_list(&ubam_list_file_local, &line)?;
        writeln!(writer, "{}\t{}\t{}", sample_id, line, ubam_list_file_remote)?;
    }
    writer.flush()?;
    Ok(())
}