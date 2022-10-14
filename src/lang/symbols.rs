use std::collections::BTreeMap;
use jati::trees::symbols::Symbols as JatiSymbols;
use jati::trees::types::Type;
use jati::trees::symbols::SymbolError;
use crate::lang::fun::FunRef;
use crate::lang::var::Var;
use crate::lang::fun::builtin;

pub(crate) struct Symbols {
    funs: BTreeMap<String, FunRef>,
}

impl Symbols {
    pub(crate) fn new() -> Symbols {
        let funs: BTreeMap<String, FunRef> = builtin::get_builtin_funs();
        Symbols { funs }
    }
}

impl JatiSymbols<Var, FunRef> for Symbols {
    fn get_var(&mut self, _name: &str) -> Result<Var, SymbolError> {
        todo!()
    }

    fn get_fun(&mut self, name: &str, args: Vec<Type>) -> Result<FunRef, SymbolError> {
        match self.funs.get(name) {
            Some(fun) => {
                match fun.fun().check_arg_types(&args) {
                    Ok(_) => { Ok(fun.clone()) }
                    Err(args_failure) => {
                        Err(SymbolError::args_issue(name, args_failure))
                    }
                }
            }
            None => { Err(SymbolError::no_such_fun(name)) }
        }
    }
}

