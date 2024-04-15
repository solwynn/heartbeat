use config::{Config, ConfigError};
use std::collections::HashMap;
use structmap::FromMap;
use structmap_derive::FromMap;

#[derive(FromMap)]
#[derive(Debug)]
struct ClientConfig {
    key: String,
    mqtt_host: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            key: String::from(""),
            mqtt_host: String::from(""),
        }
    }
}

pub fn check() {
    let conf = Config::builder()
        .add_source(config::File::with_name("config.toml"));
    
    match conf.build() {
        Ok(config) => {
            let unwrapped = config.try_deserialize::<HashMap<String, String>>().unwrap();

            let mut gm = GenericMap::new();
        
            for (key, value) in unwrapped.iter() {
                gm.insert(String::from(key), structmap::value::Value::String(String::from(value)));
            }
        
            let cc: ClientConfig = ClientConfig::from_genericmap(gm);
        
            println!("{:?}", cc);
        }

        Err(e) => match e {
            ConfigError::Frozen => todo!(),
            ConfigError::NotFound(_) => todo!(),
            ConfigError::PathParse(_) => todo!(),
            ConfigError::FileParse { uri, cause } => todo!(),
            ConfigError::Type { origin, unexpected, expected, key } => todo!(),
            ConfigError::Message(_) => todo!(),
            ConfigError::Foreign(_) => todo!(),
        }
    }


}