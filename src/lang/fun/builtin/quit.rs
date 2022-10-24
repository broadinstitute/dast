use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;
use crate::error::Error;
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::Fun;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::Runtime;
use crate::Value;

pub(crate) struct Quit {}

impl Gen for Quit {
    fn new() -> Quit { Quit {} }
}

impl Fun for Quit {
    fn tpe(&self) -> Type { Type::Unit }
    fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure> {
        check_n_args(arg_types, 0)
    }

    fn call(&self, _args: Vec<Value>, runtime: &mut Runtime) -> Result<Value, Error> {
        runtime.request_exit(Ok(Value::Unit));
        Ok(Value::Unit)
    }
}