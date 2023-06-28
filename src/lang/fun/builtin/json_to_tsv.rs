use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;
use crate::error::Error;
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::Fun;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::Runtime;
use crate::lang::value::Value;
use crate::methods::json_to_tsv::json_to_tsv;

pub(crate) struct JsonToTsv {}

impl Gen for JsonToTsv {
    fn new() -> Self { JsonToTsv {} }
}

impl Fun for JsonToTsv {
    fn tpe(&self) -> Type { Type::Unit }

    fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure> {
        check_n_args(arg_types, 0)
    }

    fn call(&self, args: Vec<Value>, runtime: &mut Runtime) -> Result<Value, Error> {
        if !args.is_empty() {
            return Err(Error::from("Fun takes no arguments"));
        }
        let env = runtime.env();
        let input = env.get_opt_arg("i")?.map(|s|s.as_str());
        let output = env.get_opt_arg("o")?.map(|s|s.as_str());
        let buffer_size =
            env.get_opt_arg("b")?.map(|s| s.parse::<usize>()).transpose()?
                .unwrap_or(1000);
        json_to_tsv(input, output, buffer_size)
    }
}