use clap::{command, Command, arg, ArgMatches};
use crate::error::Error;

pub(crate) enum Config {
    Crams(CramsConfig),
    Fastqs(FastqsConfig),
    FastqBams(FastqBamsConfig),
    Group(GroupConfig),
    Ubams(UbamsConfig),
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

pub(crate) struct UbamsConfig {
    pub(crate) input: String,
    pub(crate) target: String,
    pub(crate) prefix: String,
    pub(crate) output: String,
}

mod names {
    pub(crate) const CRAMS: &str = "crams";
    pub(crate) const FASTQS: &str = "fastqs";
    pub(crate) const FASTQ_BAMS: &str = "fastq-bams";
    pub(crate) const GROUP: &str = "group";
    pub(crate) const UBAMS: &str = "ubams";
}

fn arg_as_string(arg_matches: &ArgMatches, key: &str, name: &str) -> Result<String, Error> {
    let string =
        String::from(arg_matches.value_of(key)
            .ok_or_else(|| { Error::from(format!("Missing {} argument.", name)) })?);
    Ok(string)
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
                .arg(arg!(-v --value <FILE> "Value column")))
            .subcommand(Command::new(names::UBAMS)
                .about("Create lists of unmapped BAM files.")
                .arg(arg!(-i --input <FILE> "Input file"))
                .arg(arg!(-t --target <FILE> "Target directory for file list files."))
                .arg(arg!(-p --prefix <STRING> "Path prefix for file list files in list."))
                .arg(arg!(-o --output <FILE> "Output file")));
        let matches = app.try_get_matches()?;
        if let Some(crams_matches) = matches.subcommand_matches(names::CRAMS) {
            let input =
                arg_as_string(crams_matches, "input", "input file")?;
            Ok(Config::Crams(CramsConfig { input }))
        } else if let
        Some(fastqs_matches) = matches.subcommand_matches(names::FASTQS) {
            let input =
                arg_as_string(fastqs_matches, "input", "input file")?;
            Ok(Config::Fastqs(FastqsConfig { input }))
        } else if let
        Some(fastqs_matches) = matches.subcommand_matches(names::FASTQ_BAMS) {
            let input =
                arg_as_string(fastqs_matches, "input", "input file")?;
            Ok(Config::FastqBams(FastqBamsConfig { input }))
        } else if let
        Some(group_matches) = matches.subcommand_matches(names::GROUP) {
            let input =
                arg_as_string(group_matches, "input", "input file")?;
            let key_col =
                arg_as_string(group_matches, "key", "key")?;
            let value_col =
                arg_as_string(group_matches, "value", "value")?;
            Ok(Config::Group(GroupConfig { input, key_col, value_col }))
        } else if let
        Some(ubams_matches) = matches.subcommand_matches(names::UBAMS) {
            let input =
                arg_as_string(ubams_matches, "input", "input file")?;
            let target =
                arg_as_string(ubams_matches, "target", "target")?;
            let prefix =
                arg_as_string(ubams_matches, "prefix", "prefix")?;
            let output =
                arg_as_string(ubams_matches, "output", "output file")?;
            Ok(Config::Ubams(UbamsConfig { input, target, prefix, output }))
        } else {
            Err(Error::from(format!("Need to specify subcommand ({}, {}, {}, {} or {})",
                                    names::CRAMS, names::FASTQS, names::FASTQ_BAMS, names::GROUP,
                                    names::UBAMS)))
        }
    }
}