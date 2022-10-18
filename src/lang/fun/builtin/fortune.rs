use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;
use crate::lang::env::Env;
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::Fun;
use crate::Value;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::RunResult;

pub(crate) struct Fortune {}

impl Gen for Fortune {
    fn new() -> Fortune { Fortune {} }
}

impl Fun for Fortune {
    fn tpe(&self) -> Type { Type::String }
    fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure> {
        check_n_args(arg_types, 0)
    }
    fn call(&self, _args: Vec<Value>, _env: &Env) -> RunResult {
        let message = "Hello, world!".to_string();
        Ok(Value::String(message))
    }
}

