#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::{Builder, Target};
use log::{debug, error, info, warn};
use std::env;
use structopt::StructOpt;

mod create_spl_account;
mod create_spl_token;

fn main() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);

    builder.init();
    let args = Cli::from_args();

    if args.pattern == "create-spl-account" {
        create_spl_account::main();
    }

    if args.pattern == "create_spl_token" {
        create_spl_token::main();
    }

    info!("success!");
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
}
