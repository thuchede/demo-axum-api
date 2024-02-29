use clap::{Arg, Command};
use dotenv::dotenv;
use rest_api::commands;

pub fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let mut command = Command::new("Book Rest API")
        .version("1.0")
        .author("Thomas Huchedé <thomas.huchede@zenika.com>")
        .about("A sample application to experiment with Rust-based microservices")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.json"),
        );


    command = commands::configure(command);

    let matches = command.get_matches();

    commands::handle(&matches)?;

    Ok(())
}