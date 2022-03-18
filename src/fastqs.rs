use crate::config::FastqListConfig;
use crate::error::Error;
use fs_err::File;
use std::io::{BufReader, BufRead};
use crate::nephrotic;
use std::collections::HashMap;

const HEADER_FOR_TERRA: &str =
    "fastq1\tfastq2\tlibrary_name\tplatform_name\tplatform_unit\tentity:read_group\trun_date\tsample_id\tsequencing_center";

enum Partner {
    R1(String),
    R2(String),
}

impl Partner {
    fn to_str(&self) -> &str {
        match self {
            Partner::R1(string) => { string }
            Partner::R2(string) => { string }
        }
    }
}

const DIVIDER: &str = "_RX_";

fn to_partner(string: &str) -> Option<(String, Partner)> {
    if string.contains(".1.") {
        Some((string.replace(".1.", DIVIDER), Partner::R1(String::from(string))))
    } else if string.contains(".2.") {
        Some((string.replace(".2.", DIVIDER), Partner::R2(String::from(string))))
    } else if string.contains("_R1_") {
        Some((string.replace("_R1_", DIVIDER), Partner::R1(String::from(string))))
    } else if string.contains("_R2_") {
        Some((string.replace("_R2_", DIVIDER), Partner::R2(String::from(string))))
    } else {
        None
    }
}

fn join_pair<'a>(partner1: &'a Partner, partner2: &'a Partner)
                 -> Result<(&'a str, &'a str), Error> {
    match (partner1, partner2) {
        (Partner::R1(string1), Partner::R2(string2)) => {
            Ok((string1, string2))
        }
        (Partner::R2(string2), Partner::R1(string1)) => {
            Ok((string1, string2))
        }
        (Partner::R1(string1a), Partner::R1(string1b)) => {
            Err(Error::from(format!("Cannot pair '{}' and '{}'.", string1a, string1b)))
        }
        (Partner::R2(string2a), Partner::R2(string2b)) => {
            Err(Error::from(format!("Cannot pair '{}' and '{}'.", string2a, string2b)))
        }
    }
}

fn cannot_split_in_two<'a>(string: &'a str, divider: &'a str) -> Error {
    Error::from(format!("Cannot split {} in two pieces using {}.", string, divider))
}

fn split_in_two<'a>(string: &'a str, divider: &'a str) -> Result<(&'a str, &'a str), Error> {
    let mut iter = string.split(divider);
    let part1 = iter.next().ok_or_else(|| { cannot_split_in_two(string, divider) })?;
    let part2 = iter.next().ok_or_else(|| { cannot_split_in_two(string, divider) })?;
    if iter.next().is_some() {
        Err(cannot_split_in_two(string, divider))
    } else {
        Ok((part1, part2))
    }
}

fn print_record(key: &str, fastq1: &str, fastq2: &str) -> Result<(), Error> {
    let path =
        key.strip_prefix(nephrotic::GCS_PREFIX)
            .ok_or_else(|| {
                Error::from(
                    format!("Line {} does not start with {}.", fastq1,
                            nephrotic::GCS_PREFIX)
                )
            })?;
    let (folder, file) = split_in_two(path, "/")?;
    let (cohort, sample_id) = split_in_two(folder, "__")?;
    let (prefix, lane) = split_in_two(file, DIVIDER)?;
    let read_group = format!("{}_{}_{}_{}", cohort, sample_id, prefix, lane);
    let library_name = format!("{}_{}_{}_{}", cohort, sample_id, prefix, lane);
    let platform_name = "unknown";
    let platform_unit = "unknown";
    let run_date = "unknown";
    let sequencing_center = format!("Maybe {}", cohort);
    println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", fastq1, fastq2, library_name, platform_name,
             platform_unit, read_group, run_date, sample_id, sequencing_center);
    Ok(())
}

pub(crate) fn process_fastq_list(config: FastqListConfig) -> Result<(), Error> {
    let file = File::open(config.input)?;
    let reader = BufReader::new(file);
    println!("{}", HEADER_FOR_TERRA);
    let mut partners = HashMap::<String, Partner>::new();
    for line in reader.lines() {
        let line = line?;
        let (key, partner_this) = to_partner(&line).ok_or_else(|| {
            Error::from(
                format!("Path {} does not contain '.1.', '.2.', '_R1_' or '_R2_'.", line)
            )
        })?;
        match partners.remove(&key) {
            None => {
                partners.insert(key, partner_this);
            }
            Some(partner_that) => {
                let (fastq1, fastq2) =
                    join_pair(&partner_this, &partner_that)?;
                print_record(&key, fastq1, fastq2)?
            }
        }
    }
    if !partners.is_empty() {
        let mut partners_iter =
            partners.drain().map(|(_, partner)| -> Partner { partner });
        let mut partners_string = String::new();
        partners_string.push_str(partners_iter.next().unwrap().to_str());
        for partner in partners_iter {
            partners_string.push_str(", ");
            partners_string.push_str(partner.to_str());
        }
        return Err(
            Error::from(format!("Could not pair up the following: '{}'.", partners_string))
        );
    }
    Ok(())
}