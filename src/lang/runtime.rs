use std::fmt::{Debug, Display, Formatter};
use jati::trees::typed::call::Call;
use jati::trees::typed::tree::Tree;
use crate::lang::env::Env;
use crate::lang::fun::FunRef;
use crate::lang::value::Value;
use crate::lang::var::Var;

pub(crate) struct Runtime {
    env: Env,
    exit_has_been_requested: bool,
}

#[derive(Debug)]
pub struct RunError {
    contexts: Vec<String>,
    message: String,
}

impl From<String> for RunError {
    fn from(message: String) -> Self {
        let contexts = Vec::new();
        RunError { contexts, message }
    }
}

impl From<&str> for RunError {
    fn from(message: &str) -> Self { RunError::from(message.to_string()) }
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

pub(crate) fn map_err_run<T, E: std::error::Error>(result: Result<T, E>, name: &str)
                                                   -> Result<T, RunError> {
    result.map_err(|error| {
        let mut run_error = RunError::from(error.to_string());
        run_error.add_context(name.to_string());
        run_error
    })
}

pub(crate) type RunResult = Result<Value, RunError>;

impl Runtime {
    pub(crate) fn new(env: Env) -> Runtime {
        let exit_has_been_requested = false;
        Runtime { env, exit_has_been_requested }
    }
    pub(crate) fn evaluate(&mut self, tree: &Tree<Var, FunRef>) -> RunResult {
        match tree {
            Tree::Call(call) => { self.evaluate_call(call) }
            Tree::Var(_) => { todo!() }
            Tree::Lit(_) => { todo!() }
        }
    }
    pub(crate) fn request_exit(&mut self) { self.exit_has_been_requested = true }
    pub(crate) fn exit_has_been_requested(&self) -> bool { self.exit_has_been_requested }
    fn evaluate_call(&mut self, call: &Call<Var, FunRef>) -> RunResult {
        let mut arg_values: Vec<Value> = Vec::new();
        for arg in &call.args {
            let value = self.evaluate(arg)?;
            arg_values.push(value);
        }
        call.fun.fun().call(arg_values, &self.env)
    }
}

