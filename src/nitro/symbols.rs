use jati::trees::symbols::Symbols as JatiSymbols;
use jati::trees::types::Type;
use jati::trees::symbols::errors::{no_such_fun, wrong_number_of_args};
use jati::error::Error as JatiError;
use crate::nitro::fun;
use crate::nitro::fun::Fun;
use crate::nitro::var::Var;

pub(crate) struct Symbols {
    tsv_to_metastaar: Fun
}

impl Symbols {
    pub(crate) fn new() -> Symbols {
        let tsv_to_metastaar = fun::builtin::tsv_to_metastaar::TsvToMetastaar::new_fun();
        Symbols { tsv_to_metastaar }
    }
}

impl JatiSymbols<Var, Fun> for Symbols {
    fn get_var(&mut self, _name: &str) -> Result<Var, JatiError> {
        todo!()
    }

    fn get_fun(&mut self, name: &str, args: Vec<Type>) -> Result<Fun, JatiError> {
        let tsv_to_metastaar_name = fun::builtin::tsv_to_metastaar::NAME;
        if name == tsv_to_metastaar_name {
            if args.is_empty() {
                Ok(self.tsv_to_metastaar.clone())
            } else {
                Err(wrong_number_of_args(tsv_to_metastaar_name, args.len(), 0))
            }
        } else {
            Err(no_such_fun(fun::builtin::tsv_to_metastaar::NAME))
        }
    }
}

