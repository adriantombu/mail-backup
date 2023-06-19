use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Connections {
    pub connections: Vec<Connection>,
}

impl Connections {
    pub fn new(data: &str) -> Vec<Connection> {
        toml::from_str::<Connections>(data).unwrap().connections
    }
}

#[derive(Deserialize, Debug)]
pub struct Connection {
    pub domain: String,
    pub username: String,
    pub password: String,
}
