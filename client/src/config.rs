use std::fs::File;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub auth_secret: String,
    pub url: String,
    pub name: String,
}

impl Config {
    pub fn new() -> Self {
        serde_json::from_reader(File::open("client.json").unwrap()).unwrap()
    }
}
