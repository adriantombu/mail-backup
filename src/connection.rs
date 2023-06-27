use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Connections {
    pub connections: Vec<Connection>,
}

impl Connections {
    pub fn try_new(data: &str) -> Result<Self> {
        toml::from_str::<Connections>(data).context("Could not parse connections from config file")
    }
}

#[derive(Deserialize, Debug)]
pub struct Connection {
    pub domain: String,
    pub username: String,
    pub password: String,
}
