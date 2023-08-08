mod cli;
mod errors;

use std::process::exit;
use errors::exit_with_retcode;


fn main() {
    let args = match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(Ok(()))
        },
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };
    log::info!("{:?}", args);
}
