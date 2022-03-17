use clap::{command, Command, arg};
use crate::error::Error;

pub(crate) enum Config {
    CramList(CramListConfig),
    FastqList(FastqListConfig),
}

pub(crate) struct CramListConfig {
    pub(crate) input: String,
}

pub(crate) struct FastqListConfig {
    pub(crate) input: String,
}

mod names {
    pub(crate) const CRAMS: &str = "crams";
    pub(crate) const FASTQS: &str = "fastqs";
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
                .arg(arg!(-i --input <FILE> "Input file")));
        let matches = app.try_get_matches()?;
        if let Some(crams_matches) = matches.subcommand_matches(names::CRAMS) {
            let input = String::from(
                crams_matches.value_of("input")
                    .ok_or_else(|| { Error::from("Missing input file argument.") })?
            );
            Ok(Config::CramList(CramListConfig { input }))
        } else if let
        Some(fastqs_matches) = matches.subcommand_matches(names::FASTQS) {
            let input = String::from(
                fastqs_matches.value_of("input")
                    .ok_or_else(|| { Error::from("Missing input file argument.") })?
            );
            Ok(Config::FastqList(FastqListConfig { input }))
        } else {
            Err(Error::from(format!("Need to specify subcommand ({} or {})",
                                    names::CRAMS, names::FASTQS)))
        }
    }
}