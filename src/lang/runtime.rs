use jati::trees::typed::call::Call;
use jati::trees::typed::tree::Tree;
use crate::Error;
use crate::lang::env::Env;
use crate::lang::fun::Fun;
use crate::lang::value::Value;
use crate::lang::var::Var;

pub(crate) struct Runtime {
    env: Env
}

impl Runtime {
    pub(crate) fn new(env: Env) -> Runtime { Runtime { env } }
    pub(crate) fn evaluate(&mut self, tree: &Tree<Var, Fun>) -> Result<Value, Error> {
        match tree {
            Tree::Call(call) => { self.evaluate_call(call) }
            Tree::Var(_) => { todo!() }
            Tree::Lit(_) => { todo!() }
        }
    }

    fn evaluate_call(&mut self, call: &Call<Var, Fun>) -> Result<Value, Error> {
        let mut arg_values: Vec<Value> = Vec::new();
        for arg in &call.args {
            let value = self.evaluate(arg)?;
            arg_values.push(value);
        }
        call.fun.fun_impl().call(arg_values, &self.env)
    }
}

