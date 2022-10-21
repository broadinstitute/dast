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
    Group(GroupConfig),
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

pub(crate) struct GroupConfig {
    pub(crate) input: String,
    pub(crate) key_col: String,
    pub(crate) value_col: String,
}

mod names {
    pub(crate) const SCRIPT: &str = "script";
    pub(crate) const EVAL: &str = "eval";
    pub(crate) const SHELL: &str = "shell";
    pub(crate) const GROUP: &str = "group";
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
            .subcommand(Command::new(names::GROUP)
                .about("Group records of same key, collecting values.")
                .arg(arg!(-i --input <FILE> "Input file"))
                .arg(arg!(-k --key <FILE> "Key column"))
                .arg(arg!(-v --value <FILE> "Value column")));
        match app.try_get_matches()?.subcommand() {
            Some((names::GROUP, group_matches)) => {
                let input =
                    arg_as_string(group_matches, "input", "input file")?;
                let key_col =
                    arg_as_string(group_matches, "key", "key")?;
                let value_col =
                    arg_as_string(group_matches, "value", "value")?;
                Ok(Config::Group(GroupConfig { input, key_col, value_col }))
            }
            Some((subcommand, _)) => {
                Err(Error::from(format!(
                    "Unknown subcommand {}. Known subcommands is {})", subcommand, names::GROUP
                )))
            }
            None => {
                Err(Error::from(format!("Need to specify subcommand ({})", names::GROUP)))
            }
        }
    }
}