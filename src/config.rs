use clap::{command, Command, arg};
use crate::error::Error;

pub(crate) enum Config {
    CramList(CramListConfig)
}

pub(crate) struct CramListConfig {
    pub(crate) input: String,
}

mod names {
    pub(crate) const CRAMS: &str = "crams";
}

impl Config {
    pub(crate) fn config() -> Result<Config, Error> {
        let app = command!()
            .propagate_version(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new(names::CRAMS)
                .about("Process list of CRAM files for Nephrotic syndrome KB.")
                .arg(arg!(-i --input <FILE> "Input file")));
        let matches = app.try_get_matches()?;
        if let Some(crams_matches) = matches.subcommand_matches(names::CRAMS) {
            // crams_matches.value_of("input")
            let input = String::from(
                crams_matches.value_of("input")
                    .ok_or_else(|| { Error::from("Missing input file argument.") })?
            );
            Ok(Config::CramList(CramListConfig { input }))
        } else {
            Err(Error::from(format!("Need to specify subcommand {}", names::CRAMS)))
        }
    }
}