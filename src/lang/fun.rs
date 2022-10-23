pub(crate) mod builtin;
pub(crate) mod util;

use std::rc::Rc;
use jati::trees::types::Type;
use jati::runtime::fun::Fun as JatiFun;
use jati::trees::symbols::ArgsFailure;
use crate::lang::runtime::Runtime;
use crate::lang::value::Value;

#[derive(Clone)]
pub(crate) struct FunRef {
    pub(crate) name: String,
    fun: Rc<dyn Fun>,
}

impl FunRef {
    pub(crate) fn fun(&self) -> Rc<dyn Fun> { self.fun.clone() }
}

pub(crate) trait Fun {
    fn into_fun_ref(self, name: String) -> FunRef where Self: 'static + Sized {
        let fun = Rc::new(self);
        FunRef { name, fun }
    }
    fn tpe(&self) -> Type;
    fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure>;
    fn call(&self, args: Vec<Value>, runtime: &mut Runtime) -> RunResult;
}

impl JatiFun for FunRef {
    fn tpe(&self) -> Type { self.fun.tpe() }
}
