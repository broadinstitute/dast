use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;
use crate::error::Error;
use crate::group::group;
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::Fun;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::Runtime;
use crate::Value;

pub(crate) struct Group {}

impl Gen for Group {
    fn new() -> Self { Group {} }
}

impl Fun for Group {
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
        let key_col = env.get_arg("k")?;
        let value_col = env.get_arg("v")?;
        group(input, key_col, value_col)
    }
}