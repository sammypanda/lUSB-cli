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
            let identifier_list = match sub_m.get_one::<String>("identifiers") {
                Some(value) => value
                    .split(',') // turn into comma separated list
                    .map(|attempt| attempt.parse::<u8>().ok()) // convert to u8
                    .filter_map(|result| result) // remove non-u8
                    .collect(),
                None => Vec::new(),
            };

            println!("devices found: {:?}", identifier_list);

            let device_list = identifier_list
                .into_iter() // iter() is reference to; into_iter() changes type
                .map(|compatible| cli::cli_devices::Device::new(compatible)); // create instances for each cli_device

            for device in device_list {
                println!("Device: {:?}", device.get_index().unwrap());
            }
        }
        _ => {} // required by 'match'
    }
}