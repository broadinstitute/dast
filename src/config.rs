use clap::{command, Command, arg};
use crate::error::Error;

pub(crate) enum Config {
    Crams(CramsConfig),
    Fastqs(FastqsConfig),
    FastqBams(FastqBamsConfig),
    Group(GroupConfig),
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

pub(crate) struct GroupConfig {
    pub(crate) input: String,
    pub(crate) key_col: String,
    pub(crate) value_col: String,
}

mod names {
    pub(crate) const CRAMS: &str = "crams";
    pub(crate) const FASTQS: &str = "fastqs";
    pub(crate) const FASTQ_BAMS: &str = "fastq-bams";
    pub(crate) const GROUP: &str = "group";
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
                .arg(arg!(-i --input <FILE> "Input file")))
            .subcommand(Command::new(names::GROUP)
                .about("Group records of same key, collecting values.")
                .arg(arg!(-i --input <FILE> "Input file"))
                .arg(arg!(-k --key <FILE> "Key column"))
                .arg(arg!(-v --value <FILE> "Value column")));
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
        } else if let
        Some(group_matches) = matches.subcommand_matches(names::GROUP) {
            let input = String::from(
                group_matches.value_of("input")
                    .ok_or_else(|| { Error::from("Missing input file argument.") })?
            );
            let key_col = String::from(
                group_matches.value_of("key")
                    .ok_or_else(|| { Error::from("Missing key argument.") })?
            );
            let value_col = String::from(
                group_matches.value_of("value")
                    .ok_or_else(|| { Error::from("Missing value argument.") })?
            );
            Ok(Config::Group(GroupConfig { input, key_col, value_col }))
        } else {
            Err(Error::from(format!("Need to specify subcommand ({}, {}, {} or {})",
                                    names::CRAMS, names::FASTQS, names::FASTQ_BAMS, names::GROUP)))
        }
    }
}