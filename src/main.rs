use clap::{Arg, Command};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    user: String,
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
            Arg::new("user")
                .short('u')
                .long("user")
                .value_name("USER")
                .help("Your username")
                .required(true),
        )
        .get_matches();

    // Log parsing
    info!("Parsing command-line arguments...");
    warn!("This is a sample warning log for testing purposes.");

    // Handle the "user" argument
    if let Some(user) = matches.get_one::<String>("user") {
        info!("User provided: {user}");

        // Serialize and Deserialize example
        let config = Config {
            user: user.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let serialized = to_string(&config).unwrap();
        println!("Serialized Config: {serialized}");

        // Write to a file
        let mut file = File::create("config.json").expect("Failed to create file");
        file.write_all(serialized.as_bytes())
            .expect("Failed to write to file");

        let deserialized: Config = from_str(&serialized).unwrap();
        println!("Deserialized Config: {deserialized:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, to_string};

    #[test]
    fn test_config_serialisation() {
        let config = Config {
            user: "Test User".to_string(),
            version: "0.1.0".to_string(),
        };
        let serialized = to_string(&config).unwrap();
        assert_eq!(serialized, r#"{"user":"Test User","version":"0.1.0"}"#);

        let deserialized: Config = from_str(&serialized).unwrap();
        assert_eq!(deserialized.user, "Test User");
        assert_eq!(deserialized.version, "0.1.0");
    }
}
