mod symbols;
pub(crate) mod runtime;
pub(crate) mod value;
pub(crate) mod env;
mod var;
mod fun;

use std::io::{stdin, Stdin, stdout, Write};
use fs_err::read_to_string;
use jati::parse::parsers::id::RustIdParser;
use jati::parse::parsers::script::ScriptParser;
use jati::parse::parsers::white::DefaultWhiteSpaceParser;
use jati::parse_string;
use crate::config::{EvalConfig, ScriptConfig, ShellConfig};
use crate::error::{Error, map_err};
use crate::lang::env::Env;
use crate::lang::runtime::Runtime;
use crate::lang::symbols::Symbols;
use crate::lang::value::Value;

pub(crate) fn run_script(config: ScriptConfig) -> Result<Value, Error> {
    let ScriptConfig { script_file, env } = config;
    println!("script file: {}", script_file);
    for (key, values) in &env.args {
        println!("{}: {}", key, values.join(" "))
    }
    let script = read_to_string(&script_file)?;
    println!("=== Begin of nitro script ===\n{}\n=== End of nitro script ===", script);
    run_string(script, env)
}

pub(crate) fn evaluate_expression(config: EvalConfig) -> Result<Value, Error> {
    let EvalConfig { string, env } = config;
    run_string(string, env)
}

pub(crate) fn run_shell(config: ShellConfig) -> Result<Value, Error> {
    let ShellConfig { env } = config;
    let mut runtime = Runtime::new(env);
    let mut symbols = Symbols::new();
    let mut stdin = stdin();
    loop {
        let run_result = read_and_evaluate_line(&mut symbols, &mut runtime, &mut stdin);
        match &run_result {
            Ok(value) => { println!("{}", value); }
            Err(error) => { println!("{}", error) }
        }
        if runtime.exit_result_ref().is_some() {
            break
        }
    };
    let value = runtime.take_exit_result()?;
    Ok(value)
}

fn read_and_evaluate_line(symbols: &mut Symbols, runtime: &mut Runtime, stdin: &mut Stdin)
    -> Result<Value, Error> {
    print!("Tups> ");
    map_err(stdout().flush(), "STDOUT")?;
    let mut input = String::new();
    map_err(stdin.read_line(&mut input), "STDIN")?;
    println!("{}", input);
    let raw_tree =
        map_err(parse_string(parser(), &input), "Parsing error")?;
    let typed_tree =
        map_err(raw_tree.into_typed(symbols), "Typing error")?;
    let value = runtime.evaluate(&typed_tree)?;
    Ok(value)
}

fn run_string(script: String, env: Env) -> Result<Value, Error> {
    let raw_tree = parse_string(parser(), &script)?;
    let mut symbols = Symbols::new();
    let typed_tree = raw_tree.into_typed(&mut symbols)?;
    let mut runtime = Runtime::new(env);
    let value = map_err(runtime.evaluate(&typed_tree), "Evaluation error")?;
    Ok(value)
}

pub(crate) fn parser() -> ScriptParser {
    let ws_parser = DefaultWhiteSpaceParser::new();
    let id_parser = RustIdParser::new();
    ScriptParser::new(ws_parser, id_parser)
}
