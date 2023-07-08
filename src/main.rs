use clap::{Command, Arg};

mod cli;
use cli::cli_devices; // should basically be the proxy for every other *_devices_* module

fn main() {
    let cmd = Command::new(env!("CARGO_PKG_NAME"))
        .arg_required_else_help(true)
        .version(env!("CARGO_PKG_VERSION", "Version not set"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("list")
                .about("List all recognised USB devices"))
        .subcommand(
            Command::new("disable")
                .about("Disable the specified USB devices")
                .arg(
                    Arg::new("identifiers")
                        .value_name("IDENTIFIERS")
                        .required(true)
                        .help("Comma-separated list of identifiers")
                )
        ).get_matches();

    match cmd.subcommand() {
        Some(("list", _)) => {
            cli_devices::list();
        },
        Some(("disable", sub_m)) => {
            println!("Devices listed: {}", sub_m.get_one::<String>("identifiers").unwrap());
        }
        _ => {} // required by 'match'
    }
}