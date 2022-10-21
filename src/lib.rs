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
        Config::Group(group_config) => {
            group::group(group_config)?;
            Ok(Value::Unit)
        }
    }
}
