use jati::trees::typed::call::Call;
use jati::trees::typed::tree::Tree;
use crate::error::Error;
use crate::lang::env::Env;
use crate::lang::fun::FunRef;
use crate::lang::value::Value;
use crate::lang::var::Var;

pub(crate) struct Runtime {
    env: Env,
    exit_result: Option<Result<Value, Error>>,
}

impl Runtime {
    pub(crate) fn new(env: Env) -> Runtime {
        let exit_result: Option<Result<Value, Error>> = None;
        Runtime { env, exit_result }
    }
    pub(crate) fn env(&self) -> &Env { &self.env }
    pub(crate) fn evaluate(&mut self, tree: &Tree<Var, FunRef>) -> Result<Value, Error> {
        match tree {
            Tree::Call(call) => { self.evaluate_call(call) }
            Tree::Var(_) => { todo!() }
            Tree::Lit(_) => { todo!() }
        }
    }
    pub(crate) fn request_exit(&mut self, exit_result: Result<Value, Error>) {
        self.exit_result = Some(exit_result)
    }
    pub(crate) fn exit_result_ref(&self) -> &Option<Result<Value, Error>> { &self.exit_result }
    pub(crate) fn take_exit_result(&mut self) -> Result<Value, Error> {
        match self.exit_result.take() {
            None => { Ok(Value::Unit )}
            Some(result) => { result }
        }
    }
    fn evaluate_call(&mut self, call: &Call<Var, FunRef>) -> Result<Value, Error> {
        let mut arg_values: Vec<Value> = Vec::new();
        for arg in &call.args {
            let value = self.evaluate(arg)?;
            arg_values.push(value);
        }
        call.fun.fun().call(arg_values, self)
    }
}

