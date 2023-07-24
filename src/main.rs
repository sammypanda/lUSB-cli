use clap::{Command, Arg};
use core::iter::Iterator;

mod cli;
use cli::cli_devices; // should basically be the proxy for every other *_devices_* module

// responsible for parsing command inputs and delegating tasks
fn main() {
    let identifiers_arg: Arg = Arg::new("identifiers")
        .value_name("IDENTIFIERS")
        .required(true)
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
            )
        .get_matches();

    match cmd.subcommand() {
        Some(("list", _)) => {
            cli_devices::list();
        },
        Some((verb, sub_m)) => {
            let device_list = match sub_m.get_one::<String>("identifiers") {
                Some(value) => device_iter(value),
                None => panic!("Missing list of identifiers"),
            };

            for device in device_list {
                cli_devices::handle_verb(verb, &device)
            };
        },
        _ => {} // required by 'match'
    };

    fn device_iter(list: &str) -> impl Iterator<Item = cli_devices::Device> + '_ {
        list
            .split(',')
            .filter_map(|attempt| {
                if let Ok(compatible) = attempt.parse::<u8>() {
                    Some(cli::cli_devices::Device::new(compatible))
                } else {
                    None
                }
            })
    }
}