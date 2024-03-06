mod hello;
mod serve;
mod migrate;

use clap::{ArgMatches, Command};
use crate::settings::Settings;


pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
        .subcommand(migrate::configure())
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, settings)?;
    serve::handle(matches, settings)?;
    migrate::handle(matches, settings)?;

    Ok(())
}