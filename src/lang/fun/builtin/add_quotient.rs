use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;
use crate::data::line_parser::LineParser;
use crate::error::Error;
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::Fun;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::Runtime;
use crate::lang::value::Value;
use crate::methods::add_quotient::add_quotient;

pub(crate) struct AddQuotient {}

impl Gen for AddQuotient {
    fn new() -> Self { AddQuotient {} }
}

impl Fun for AddQuotient {
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
        let numerator = env.get_arg("p")?;
        let denominator = env.get_arg("q")?;
        let col_name =
            env.get_opt_arg("n")?.map(|s|s.as_str()).unwrap_or("quot");
        let line_parser =
            env.get_opt_arg("f")?.map(|s| LineParser::from_name(s))
                .transpose()?.unwrap_or(LineParser::new_tsv());
        add_quotient(input, output, numerator, denominator, col_name, line_parser)
    }
}