use fs_err::read_to_string;
use crate::config::NitroConfig;
use crate::Error;

pub(crate) fn nitro(config: NitroConfig) -> Result<(), Error> {
    println!("script file: {}", config.script_file);
    for (key, values) in config.args {
        println!("{}: {}", key, values.join(" "))
    }
    let script = read_to_string(&config.script_file)?;
    todo!()
}