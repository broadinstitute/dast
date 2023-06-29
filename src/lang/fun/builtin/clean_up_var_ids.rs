use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;
use crate::data::io::line_parser::LineParser;
use crate::error::Error;
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::Fun;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::Runtime;
use crate::lang::value::Value;
use crate::methods::clean_up_var_ids::clean_up_var_ids;

pub(crate) struct CleanUpVarIds {}

impl Gen for CleanUpVarIds {
    fn new() -> Self { CleanUpVarIds {} }
}

impl Fun for CleanUpVarIds {
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
        let line_parser =
            env.get_opt_arg("f")?.map(|s| LineParser::from_name(s))
                .transpose()?.unwrap_or(LineParser::new_tsv());
        clean_up_var_ids(input, output, line_parser)
    }
}