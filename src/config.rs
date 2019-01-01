use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub crypto_iso: String,
    pub fiat_iso: String,
    pub crypto_logo: String
}

pub fn get_config() -> Config {
    let mut file = File::open("conf.toml").expect("Can't read file");
    let mut config = String::new();
    file.read_to_string(&mut config)
        .expect("Could not read file");
    
    let config: Config = toml::from_str(&config).unwrap();
    config
}
