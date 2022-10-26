use methods::group;
use crate::config::Config;
use crate::error::Error;
use crate::lang::value::Value;

mod config;
mod error;
mod col_names;
mod lang;
mod data;
mod methods;
mod about;

pub fn run() -> Result<Value, Error> {
    let config = Config::new()?;
    match config {
        Config::Script(nitro_config) => {
            lang::run_script(nitro_config)
        }
        Config::Eval(eval_config) => {
            lang::evaluate_expression(eval_config)
        }
        Config::Shell(shell_config) => {
            lang::run_shell(shell_config)
        }
        Config::Version => {
            let version = format!("{}", about::name_and_version()
                .unwrap_or_else(|| "No version available".to_string()));
            Ok(Value::String(version))
        }
        Config::Help => {
            Ok(Value::String(about::about()))
        }
    }
}
