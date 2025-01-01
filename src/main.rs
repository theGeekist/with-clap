use clap::{Arg, Command};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    version: String,
}

fn main() {
    // Initialize logging
    pretty_env_logger::init();
    info!("Starting the application...");

    // Parse CLI arguments
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A starter CLI application using Rust")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Your name")
                .takes_value(true),
        )
        .get_matches();

    if let Some(name) = matches.value_of("name") {
        info!("Name provided: {}", name);

        // Serialize and Deserialize example
        let config = Config {
            name: name.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        println!("Serialized Config: {}", serialized);

        let deserialized: Config = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized Config: {:?}", deserialized);
    } else {
        warn!("No name provided. Use --help for usage.");
    }
}
