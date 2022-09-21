use std::rc::Rc;
use jati::trees::types::Type;
use crate::Error;
use crate::nitro::env::Env;
use crate::nitro::fun::{Fun, FunImpl};
use crate::nitro::value::Value;

pub(crate) struct TsvToMetastaar {}

pub(crate) const NAME: &str = "tsv_to_metastaar";

impl TsvToMetastaar {
    pub(crate) fn new_fun() -> Fun {
        let fun_impl = Rc::new(TsvToMetastaar {});
        let tpe = Type::Unit;
        Fun { fun_impl, tpe }
    }
}

fn ensure_single_arg<'a>(env: &'a Env, key: &str) -> Result<&'a String, Error> {
    match env.args.get(key) {
        None => { Err(Error::from(format!("Missing argument {}.", key))) }
        Some(values) => {
            if values.len() > 1 {
                Err(Error::from(format!(
                    "Argument {} should have exactly one value, but has {}.", key,
                    values.len())
                ))
            } else {
                values.first().ok_or_else(|| {
                    Error::from(format!("Missing argument {}", key))
                })
            }
        }
    }
}

impl FunImpl for TsvToMetastaar {
    fn call(&self, args: Vec<Value>, env: &Env) -> Result<Value, Error> {
        if !args.is_empty() {
            return Err(Error::from(format!("{} takes no parameters.", NAME)));
        }
        let input_file_name = ensure_single_arg(env, "i")?;
        println!("Yo! At least got this far!");
        todo!()
    }
}