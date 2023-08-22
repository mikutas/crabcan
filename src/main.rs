mod cli;
mod errors;
mod container;
mod config;

use std::process::exit;
use errors::exit_with_retcode;
#[macro_use] extern crate scan_fmt;

fn main() {
    let args = match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args))
        },
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };
    log::info!("{:?}", args);
}
