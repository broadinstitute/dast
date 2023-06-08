use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;
use crate::error::Error;
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::Fun;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::Runtime;
use crate::lang::value::Value;
use crate::methods::subset_wilcox::subset_wilcox;

pub(crate) struct SubsetWilcox {}

impl Gen for SubsetWilcox {
    fn new() -> Self { SubsetWilcox {} }
}

impl Fun for SubsetWilcox {
    fn tpe(&self) -> Type { Type::Unit }
    fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure> {
        check_n_args(arg_types, 0)
    }
    fn call(&self, args: Vec<Value>, runtime: &mut Runtime) -> Result<Value, Error> {
        if !args.is_empty() {
            return Err(Error::from("Fun takes no arguments"));
        }
        let env = runtime.env();
        let ranks_file = env.get_arg("ranks-file")?;
        let ranks_file_col = env.get_arg("ranks-file-col")?;
        let subset_file = env.get_arg("subset-file")?;
        subset_wilcox(ranks_file, ranks_file_col, subset_file)
    }
}