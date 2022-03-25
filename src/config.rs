use clap::{command, Command, arg};
use crate::error::Error;

pub(crate) enum Config {
    Crams(CramsConfig),
    Fastqs(FastqsConfig),
    FastqBams(FastqBamsConfig),
}

pub(crate) struct CramsConfig {
    pub(crate) input: String,
}

pub(crate) struct FastqsConfig {
    pub(crate) input: String,
}

pub(crate) struct FastqBamsConfig {
    pub(crate) input: String,
}

mod names {
    pub(crate) const CRAMS: &str = "crams";
    pub(crate) const FASTQS: &str = "fastqs";
    pub(crate) const FASTQ_BAMS: &str = "fastq-bams";
}

impl Config {
    pub(crate) fn new() -> Result<Config, Error> {
        let app = command!()
            .propagate_version(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new(names::CRAMS)
                .about("Process list of CRAM files for Nephrotic syndrome KB.")
                .arg(arg!(-i --input <FILE> "Input file")))
            .subcommand(Command::new(names::FASTQS)
                .about("Process list of FASTQ files for Nephrotic syndrome KB.")
                .arg(arg!(-i --input <FILE> "Input file")))
            .subcommand(Command::new(names::FASTQ_BAMS)
                .about("Process list of FASTQ-derived BAM files for Nephrotic syndrome KB.")
                .arg(arg!(-i --input <FILE> "Input file")));
        let matches = app.try_get_matches()?;
        if let Some(crams_matches) = matches.subcommand_matches(names::CRAMS) {
            let input = String::from(
                crams_matches.value_of("input")
                    .ok_or_else(|| { Error::from("Missing input file argument.") })?
            );
            Ok(Config::Crams(CramsConfig { input }))
        } else if let
        Some(fastqs_matches) = matches.subcommand_matches(names::FASTQS) {
            let input = String::from(
                fastqs_matches.value_of("input")
                    .ok_or_else(|| { Error::from("Missing input file argument.") })?
            );
            Ok(Config::Fastqs(FastqsConfig { input }))
        } else if let
        Some(fastqs_matches) = matches.subcommand_matches(names::FASTQ_BAMS) {
            let input = String::from(
                fastqs_matches.value_of("input")
                    .ok_or_else(|| { Error::from("Missing input file argument.") })?
            );
            Ok(Config::FastqBams(FastqBamsConfig { input }))
        } else {
            Err(Error::from(format!("Need to specify subcommand ({}, {} or {})",
                                    names::CRAMS, names::FASTQS, names::FASTQ_BAMS)))
        }
    }
}