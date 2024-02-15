use serde::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub motd: String,
    pub address: String,
}

impl Config {
    pub fn read() -> Self {
        toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap()
    }
}
