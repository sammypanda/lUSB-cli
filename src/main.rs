use clap::{Command, Arg};

mod cli;
use cli::cli_devices; // should basically be the proxy for every other *_devices_* module

// responsible for parsing command inputs and delegating tasks
fn main() {
    let identifiers_arg: Arg = Arg::new("identifiers")
        .value_name("IDENTIFIERS")
        .required(true)
        .value_delimiter(',')
        .value_parser(clap::value_parser!(u8))
        .help("Comma-separated list of identifiers");

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
                    &identifiers_arg
                )
            )
        .subcommand(
            Command::new("enable")
                .about("Enable the specified USB devices")
                .arg(
                    &identifiers_arg
                )
            );

    let cmd_result = cmd.clone().get_matches();

    match cmd_result.subcommand() {
        Some(("list", _)) => {
            cli_devices::list();
        },
        Some((verb, sub_m)) => {
            let identifiers = sub_m.get_many::<u8>("identifiers") // we can be sure it exists since `clap` handles parsing
                .unwrap_or_else(|| panic!("Comma-separted identifiers not found")); // ..but just in case

            for device in identifiers {
                cli_devices::handle_verb(verb, &cli::cli_devices::Device::new(*device))
            };
        },
        _ => {} // required by 'match'
    };
}