use std::fmt::{Debug, Display, Formatter};
use jati::trees::typed::call::Call;
use jati::trees::typed::tree::Tree;
use crate::lang::env::Env;
use crate::lang::fun::FunRef;
use crate::lang::value::Value;
use crate::lang::var::Var;

pub(crate) struct Runtime {
    env: Env,
    exit_result: Option<RunResult>,
}

impl From<String> for RunError {
    fn from(message: String) -> Self {
        let contexts = Vec::new();
        RunError { contexts, message }
    }
}

impl RunError {
    pub(crate) fn add_context(&mut self, context: String) {
        self.contexts.push(context)
    }
    pub(crate) fn report(&self) -> String {
        let mut report = String::new();
        for context in self.contexts.iter().rev() {
            report.push_str(context);
            report.push_str(": ")
        }
        report.push_str(&self.message);
        report
    }
}

impl Display for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { writeln!(f, "{}", self.report()) }
}

impl std::error::Error for RunError {}

impl Runtime {
    pub(crate) fn new(env: Env) -> Runtime {
        let exit_result: Option<RunResult> = None;
        Runtime { env, exit_result }
    }
    pub(crate) fn env(&self) -> &Env { &self.env }
    pub(crate) fn evaluate(&mut self, tree: &Tree<Var, FunRef>) -> RunResult {
        match tree {
            Tree::Call(call) => { self.evaluate_call(call) }
            Tree::Var(_) => { todo!() }
            Tree::Lit(_) => { todo!() }
        }
    }
    pub(crate) fn request_exit(&mut self, exit_result: RunResult) {
        self.exit_result = Some(exit_result)
    }
    pub(crate) fn exit_result_ref(&self) -> &Option<RunResult> { &self.exit_result }
    pub(crate) fn take_exit_result(&mut self) -> RunResult {
        match self.exit_result.take() {
            None => { Ok(Value::Unit )}
            Some(result) => { result }
        }
    }
    fn evaluate_call(&mut self, call: &Call<Var, FunRef>) -> RunResult {
        let mut arg_values: Vec<Value> = Vec::new();
        for arg in &call.args {
            let value = self.evaluate(arg)?;
            arg_values.push(value);
        }
        call.fun.fun().call(arg_values, self)
    }
}

