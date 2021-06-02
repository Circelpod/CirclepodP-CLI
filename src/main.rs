#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::{Builder, Target};
use log::{debug, error, info, warn};
use std::env;
use structopt::StructOpt;
use std::num::ParseIntError;

mod create_spl_account;
mod create_spl_token;

fn main() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);

    builder.init();
    let args = Cli::from_args();

    match args.cmd {
        Command::SPL(value) => match value.spl_operating {
            SPLOperating::CreateToken(info) => create_spl_token::main(info.decimals),
            SPLOperating::CreateAccount => create_spl_account::main(),
        },
    }

    info!("success!");
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// SPL
    SPL(SPL),
}

#[derive(Debug, StructOpt)]
struct SPL {
    /// SPL Operating
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    spl_operating: SPLOperating,
}

// subsubcommand!
#[derive(Debug, StructOpt)]
enum SPLOperating {
    /// Create Token
    CreateToken(CreateToken),
    /// Create Account
    CreateAccount,
}

#[derive(Debug, StructOpt)]
struct CreateToken {
    /// decimals
    #[structopt(short, default_value = "6", parse(try_from_str = parse_hex))]
    decimals: u8,
}

fn parse_hex(src: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(src, 16)
}
