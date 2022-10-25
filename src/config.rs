use std::env::args;
use crate::about;
use crate::error::Error;
use crate::lang::env::Env;

pub(crate) enum Choice {
    Script,
    Eval,
    Shell,
    Version,
    Help,
}

pub(crate) enum Config {
    Eval(EvalConfig),
    Script(ScriptConfig),
    Shell(ShellConfig),
    Version,
    Help,
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

mod names {
    pub(crate) const SCRIPT: &str = "script";
    pub(crate) const EVAL: &str = "eval";
    pub(crate) const SHELL: &str = "shell";
    pub(crate) const VERSION: &str = "version";
    pub(crate) const HELP: &str = "help";
    pub(crate) const NEED: &str = "Need to specify: script, eval, shell, version, or help.";
}


fn subcommand_to_choice() -> Result<Choice, Error> {
    if let Some(subcommand) = args().nth(1) {
        match subcommand.as_str() {
            names::SCRIPT => { Ok(Choice::Script) }
            names::EVAL => { Ok(Choice::Eval) }
            names::SHELL => { Ok(Choice::Shell) }
            names::VERSION => { Ok(Choice::Version) }
            names::HELP => { Ok(Choice::Help) }
            subcommand => {
                Err(Error::from(format!("Unknown subcommand '{}'. {}",
                                        subcommand, names::NEED)))
            }
        }
    } else {
        Err(Error::from(format!("Missing subcommand. {}", names::NEED)))
    }
}

impl Config {
    pub(crate) fn new() -> Result<Config, Error> {
        match subcommand_to_choice()? {
            Choice::Script => { Ok(Config::Script(Config::new_script_config()?)) }
            Choice::Eval => { Ok(Config::Eval(Config::new_eval_config()?)) }
            Choice::Shell => { Ok(Config::Shell(Config::new_shell_config()?)) }
            Choice::Version => { Ok(Config::Version) }
            Choice::Help => { Ok(Config::Help) }
        }
    }
    fn new_script_config() -> Result<ScriptConfig, Error> {
        let mut args = args();
        let script_file =
            args.nth(2).ok_or_else(|| {
                Error::from(format!("Missing script file argument.\n{}", about::USAGE))
            })?;
        let env = Env::new();
        Ok(ScriptConfig { script_file, env })
    }
    fn new_eval_config() -> Result<EvalConfig, Error> {
        let mut args = args();
        let string =
            args.nth(2).ok_or_else(|| {
                Error::from(format!("Missing expression argument.\n{}", about::USAGE))
            })?;
        let env = Env::new();
        Ok(EvalConfig { string, env })
    }
    fn new_shell_config() -> Result<ShellConfig, Error> {
        let env = Env::new();
        Ok(ShellConfig { env })
    }
}