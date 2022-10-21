use std::env::args;
use clap::{command, Command, arg, ArgMatches, Arg};
use crate::error::Error;
use crate::lang::env::Env;

pub(crate) enum Choice {
    Script,
    Eval,
    Shell
}

pub(crate) enum Config {
    Eval(EvalConfig),
    Script(ScriptConfig),
    Shell(ShellConfig),
    Fastqs(FastqsConfig),
    FastqBams(FastqBamsConfig),
    Group(GroupConfig),
    Ubams(UbamsConfig),
}

pub(crate) struct ScriptConfig {
    pub(crate) script_file: String,
    pub(crate) env: Env,
}

pub(crate) struct EvalConfig {
    pub(crate) string: String,
    pub(crate) env: Env,
}

pub(crate) struct ShellConfig {
    pub(crate) env: Env,
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
    pub(crate) const SCRIPT: &str = "script";
    pub(crate) const EVAL: &str = "eval";
    pub(crate) const SHELL: &str = "shell";
    pub(crate) const FASTQS: &str = "fastqs";
    pub(crate) const FASTQ_BAMS: &str = "fastq-bams";
    pub(crate) const GROUP: &str = "group";
    pub(crate) const UBAMS: &str = "ubams";
    pub(crate) const VARARG: &str = "vararg";
}

fn arg_as_string(arg_matches: &ArgMatches, key: &str, name: &str) -> Result<String, Error> {
    let string =
        String::from(arg_matches.value_of(key)
            .ok_or_else(|| { Error::from(format!("Missing {} argument.", name)) })?);
    Ok(string)
}

fn subcommand_to_tups_choice() -> Option<Choice> {
    if let Some(subcommand) = args().nth(1) {
        match subcommand.as_str() {
            names::SCRIPT => { Some(Choice::Script) }
            names::EVAL => { Some(Choice::Eval) }
            names::SHELL => { Some(Choice::Shell) }
            _ => { None }
         }
    } else {
        None
    }
}

impl Config {
    pub(crate) fn new() -> Result<Config, Error> {
        match subcommand_to_tups_choice() {
            Some(choice) => {
                match choice {
                    Choice::Script => { Ok(Config::Eval(Config::new_eval_config()?)) }
                    Choice::Eval => { Ok(Config::Script(Config::new_script_config()?)) }
                    Choice::Shell => { Ok(Config::Shell(Config::new_shell_config()?)) }
                }
            }
            None => {
                Config::new_clap_parsed()
            }
        }
    }
    pub fn new_script_config() -> Result<ScriptConfig, Error> {
        let mut args = args();
        let script_file =
            args.nth(2).ok_or_else(|| {
                Error::from("Missing script file argument.")
            })?;
        let env = Env::new();
        Ok(ScriptConfig { script_file, env })
    }
    pub fn new_eval_config() -> Result<EvalConfig, Error> {
        let mut args = args();
        let string =
            args.nth(2).ok_or_else(|| {
                Error::from("Missing expression argument.")
            })?;
        let env = Env::new();
        Ok(EvalConfig { string, env })
    }
    pub fn new_shell_config() -> Result<ShellConfig, Error> {
        let env = Env::new();
        Ok(ShellConfig { env })
    }
    pub fn new_clap_parsed() -> Result<Config, Error> {
        let app = command!()
            .propagate_version(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new(names::EVAL)
                .about("Evaluate expression")
                .trailing_var_arg(true)
                .arg(Arg::new(names::VARARG))
            )
            .subcommand(Command::new(names::SCRIPT)
                .about("Execute script")
                .trailing_var_arg(true)
                .arg(Arg::new(names::VARARG))
            )
            .subcommand(Command::new(names::SHELL)
                .about("Run shell")
                .trailing_var_arg(true)
                .arg(Arg::new(names::VARARG))
            )
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
        match app.try_get_matches()?.subcommand() {
            Some((names::FASTQS, fastqs_matches)) => {
                let input =
                    arg_as_string(fastqs_matches, "input", "input file")?;
                Ok(Config::Fastqs(FastqsConfig { input }))
            }
            Some((names::FASTQ_BAMS, fastqs_matches)) => {
                let input =
                    arg_as_string(fastqs_matches, "input", "input file")?;
                Ok(Config::FastqBams(FastqBamsConfig { input }))
            }
            Some((names::GROUP, group_matches)) => {
                let input =
                    arg_as_string(group_matches, "input", "input file")?;
                let key_col =
                    arg_as_string(group_matches, "key", "key")?;
                let value_col =
                    arg_as_string(group_matches, "value", "value")?;
                Ok(Config::Group(GroupConfig { input, key_col, value_col }))
            }
            Some((names::UBAMS, ubams_matches)) => {
                let input =
                    arg_as_string(ubams_matches, "input", "input file")?;
                let target =
                    arg_as_string(ubams_matches, "target", "target")?;
                let prefix =
                    arg_as_string(ubams_matches, "prefix", "prefix")?;
                let output =
                    arg_as_string(ubams_matches, "output", "output file")?;
                Ok(Config::Ubams(UbamsConfig { input, target, prefix, output }))
            }
            Some((subcommand, _)) => {
                Err(Error::from(format!(
                    "Unknown subcommand {}. Known subcommands are {}, {}, {} and {})",
                    subcommand, names::FASTQS, names::FASTQ_BAMS, names::GROUP, names::UBAMS
                )))
            }
            None => {
                Err(Error::from(format!("Need to specify subcommand ({}, {}, {} or {})",
                                        names::FASTQS, names::FASTQ_BAMS, names::GROUP,
                                        names::UBAMS)))
            }
        }
    }
}