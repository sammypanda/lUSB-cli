use clap::{Command};

mod cli;

fn main() {
    let mut cmd = Command::new(env!("CARGO_PKG_NAME"))
        .arg_required_else_help(true)
        .version(env!("CARGO_PKG_VERSION", "Version not set"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("list")
                .about("List all recognised USB devices")
        );

    let matches = cmd.get_matches_mut();

    match matches.subcommand_name() {
        Some("list") => {
            cli::cli_devices_list::demo();
        },
        _ => {} // required by 'match'
    }
}