use jati::trees::symbols::Symbols as JatiSymbols;
use jati::trees::types::Type;
use jati::trees::symbols::errors::{no_such_fun, wrong_number_of_args};
use jati::error::Error as JatiError;
use crate::lang::fun;
use crate::lang::fun::Fun;
use crate::lang::var::Var;

pub(crate) struct Symbols {
    munge_for_metastaar: Fun
}

impl Symbols {
    pub(crate) fn new() -> Symbols {
        let munge_for_metastaar = fun::builtin::munge_for_metastaar::MungeForMetastaar::new_fun();
        Symbols { munge_for_metastaar }
    }
}

impl JatiSymbols<Var, Fun> for Symbols {
    fn get_var(&mut self, _name: &str) -> Result<Var, JatiError> {
        todo!()
    }

    fn get_fun(&mut self, name: &str, args: Vec<Type>) -> Result<Fun, JatiError> {
        let munge_for_metastaar_name = fun::builtin::munge_for_metastaar::NAME;
        if name == munge_for_metastaar_name {
            if args.is_empty() {
                Ok(self.munge_for_metastaar.clone())
            } else {
                Err(wrong_number_of_args(munge_for_metastaar_name, args.len(), 0))
            }
        } else {
            Err(no_such_fun(name))
        }
    }
}

