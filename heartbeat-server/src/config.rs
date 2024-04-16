use config::{Config, ConfigError};
use serde::Serialize;
use std::collections::HashMap;
use std::io::{self, Write};
use std::fs::File;
use std::{process, env};
use structmap::FromMap;
use structmap_derive::FromMap;
use toml;

#[derive(FromMap)]
#[derive(Debug)]
#[derive(Serialize)]
#[derive(Clone)]
pub struct ServerConfig {
    pub mqtt_host: String,
    pub mqtt_port: i64,
    pub mqtt_topic: String,
    pub enable_discord_integration: bool,
    pub discord_api_key: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            mqtt_host: String::from("localhost"),
            mqtt_port: 1883,
            mqtt_topic: String::from("test-topic"),
            enable_discord_integration: false,
            discord_api_key: String::from(""),
        }
    }
}

pub fn get() -> ServerConfig {
    let cc: ServerConfig;
    let mut exedirbuf = env::current_exe().unwrap();
    exedirbuf.pop();
    exedirbuf.push("config.toml");

    let exedir = exedirbuf.to_str().unwrap();

    let conf = Config::builder()
        .add_source(config::File::with_name(exedir));
    
    match conf.build() {
        Ok(config) => {
            let unwrapped = config.try_deserialize::<HashMap<String, String>>().unwrap();

            let mut gm = GenericMap::new();
        
            for (key, value) in unwrapped.iter() {
                gm.insert(String::from(key), structmap::value::Value::String(String::from(value)));
            }
        
            cc = ServerConfig::from_genericmap(gm);
        
            return cc;
        }

        Err(e) => match e {
            ConfigError::Frozen => panic!("Frozen: {e:#?}"),
            ConfigError::NotFound(_) => panic!("NotFound: {e:#?}"),
            ConfigError::PathParse(_) => panic!("PathParse: {e:#?}"),
            ConfigError::FileParse { uri, cause } => panic!("FileParse: uri: {uri:#?}, cause: {cause:#?}"),
            ConfigError::Type { origin, unexpected, expected, key } 
                => panic!("Type -- Origin: {:#?}, unexpected: {:#?}, expected: {:#?}, key: {:#?}", origin, unexpected, expected, key),
            ConfigError::Message(_) => panic!("Message: {e:#?}"),
            ConfigError::Foreign(msg) => {
                if msg.to_string().contains("not found") {
                    let mut default = String::new();
                    let mut cont = String::new();

                    print!("config.toml not found, generate one now? [Y/N]: ");
                    io::stdout().flush().expect("Error flushing stdout");
                    io::stdin().read_line(&mut default).expect("Error reading stdin");

                    if default.trim().to_lowercase() == "n" { process::exit(1); }
                    println!("Generating config.toml...");

                    cc = ServerConfig::default();

                    let toml = toml::to_string(&cc).unwrap();

                    let mut f = File::create_new(exedir).unwrap();

                    f.write(toml.as_bytes()).unwrap();

                    print!("Continue client with default configuration values? (Note: this may only be suitable for development environments) [Y/N]: ");
                    io::stdout().flush().expect("Error flushing stdout");
                    io::stdin().read_line(&mut cont).expect("Error reading stdin");

                    if cont.trim().to_lowercase() == "n" { process::exit(1); }
                    println!("Continuing execution...");

                    return cc;
                } else {
                    panic!("{msg}");
                }
            }
        }
    }
}