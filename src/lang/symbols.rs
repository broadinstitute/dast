use jati::trees::symbols::Symbols as JatiSymbols;
use jati::trees::types::Type;
use jati::trees::symbols::SymbolError;
use crate::lang::fun::Fun;
use crate::lang::var::Var;
use crate::lang::fun::builtin;

pub(crate) struct Symbols {
    munge_for_metastaar: Fun,
}

impl Symbols {
    pub(crate) fn new() -> Symbols {
        let munge_for_metastaar = builtin::munge_for_metastaar::MungeForMetastaar::new_fun();
        Symbols { munge_for_metastaar }
    }
}

impl JatiSymbols<Var, Fun> for Symbols {
    fn get_var(&mut self, _name: &str) -> Result<Var, SymbolError> {
        todo!()
    }

    fn get_fun(&mut self, name: &str, args: Vec<Type>) -> Result<Fun, SymbolError> {
        let munge_for_metastaar_name = builtin::munge_for_metastaar::NAME;
        if name == munge_for_metastaar_name {
            let fun = self.munge_for_metastaar.clone();
            match fun.fun_impl().check_arg_types(&args) {
                Ok(_) => { Ok(fun) }
                Err(args_failure) => { Err(SymbolError::args_issue(name, args_failure)) }
            }
        } else {
            Err(SymbolError::no_such_fun(name))
        }
    }
}

