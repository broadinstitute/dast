use crate::config::Config;
use crate::error::Error;

mod config;
mod error;
mod crams;

pub fn run() -> Result<(), Error> {
    let config = Config::config()?;
    match config {
        Config::CramList(cram_list_config) => {
            crams::process_cram_list(cram_list_config)?
        }
    }
    Ok(())
}
