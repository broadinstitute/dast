pub(crate) mod builtin;

use std::rc::Rc;
use jati::trees::types::Type;
use jati::engine::fun::Fun as JatiFun;
use crate::Error;
use crate::lang::env::Env;
use crate::lang::value::Value;

#[derive(Clone)]
pub(crate) struct Fun {
    fun_impl: Rc<dyn FunImpl>,
    tpe: Type,
}

impl Fun {
    pub(crate) fn fun_impl(&self) -> Rc<dyn FunImpl> { self.fun_impl.clone() }
}

pub(crate) trait FunImpl {
    fn call(&self, args: Vec<Value>, env: &Env) -> Result<Value, Error>;
}

impl JatiFun for Fun {
    fn tpe(&self) -> Type { self.tpe }
}
