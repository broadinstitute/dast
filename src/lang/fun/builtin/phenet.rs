use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;
use crate::error::Error;
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::Fun;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::Runtime;
use crate::lang::value::Value;
use crate::methods::phenet::phenet;

pub(crate) struct Phenet {}

impl Gen for Phenet {
    fn new() -> Self { Phenet {} }
}

impl Fun for Phenet {
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
        let output = env.get_arg("o")?;
        let z_threshold =
            match env.get_opt_arg("z")? {
                None => { 0.0 }
                Some(arg) => { arg.parse::<f64>()? }
            };
         phenet(input, output, z_threshold)
    }
}