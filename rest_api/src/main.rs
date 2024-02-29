use clap::{Arg, Command};
use dotenv::dotenv;
use rest_api::{commands, settings};

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

    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    let settings = settings::Settings::new(config_location, "BOOK")?;

    commands::handle(&matches, &settings)?;

    println!(
        "db url: {}",
        settings
            .database
            .url
            .unwrap_or("missing database url".to_string())
    );

    println!(
        "log level: {}",
        settings.logging.log_level.unwrap_or("info".to_string())
    );

    // commands::handle(&matches)?;

    Ok(())
}