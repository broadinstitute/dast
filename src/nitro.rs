mod symbols;
mod runtime;
mod value;
pub(crate) mod env;
mod var;
mod fun;

use fs_err::read_to_string;
use jati::parse::parsers::id::RustIdParser;
use jati::parse::parsers::script::ScriptParser;
use jati::parse::parsers::white::DefaultWhiteSpaceParser;
use jati::parse_string;
use crate::config::NitroConfig;
use crate::Error;
use crate::nitro::runtime::Runtime;
use crate::nitro::symbols::Symbols;

pub(crate) fn nitro(config: NitroConfig) -> Result<(), Error> {
    let NitroConfig { script_file, env} = config;
    println!("script file: {}", script_file);
    for (key, values) in &env.args {
        println!("{}: {}", key, values.join(" "))
    }
    let script = read_to_string(&script_file)?;
    println!("=== Begin of nitro script ===\n{}\n=== End of nitro script ===", script);
    let raw_tree = parse_string(parser(), &script)?;
    let mut symbols = Symbols::new();
    let typed_tree = raw_tree.into_typed(&mut symbols)?;
    let mut engine = Runtime::new(env);
    let value = engine.evaluate(&typed_tree)?;
    println!("{}", value);
    Ok(())
}

pub(crate) fn parser() -> ScriptParser {
    let ws_parser = DefaultWhiteSpaceParser::new();
    let id_parser = RustIdParser::new();
    ScriptParser::new(ws_parser, id_parser)
}
