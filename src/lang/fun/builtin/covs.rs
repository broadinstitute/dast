use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;
use crate::error::Error;
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::Fun;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::Runtime;
use crate::lang::value::Value;
use crate::methods::covs::covs;

pub(crate) struct Covs {}

impl Gen for Covs {
    fn new() -> Self { Covs {} }
}

impl Fun for Covs {
    fn tpe(&self) -> Type { Type::Unit }

    fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure> {
        check_n_args(arg_types, 0)
    }
    fn call(&self, args: Vec<Value>, runtime: &mut Runtime) -> Result<Value, Error> {
        if !args.is_empty() {
            return Err(Error::from("Fun takes no arguments"));
        }
        let env = runtime.env();
        let input = env.get_arg("i")?;
        let cols = env.get_args("c")?;
        covs(input, cols)
    }
}