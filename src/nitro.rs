use crate::config::NitroConfig;
use crate::Error;

pub(crate) fn nitro(config: NitroConfig) -> Result<(), Error> {
    println!("script file: {}", config.script_file);
    for (key, values) in config.args {
        println!("{}: {}", key, values.join(" "))
    }
    todo!()
}