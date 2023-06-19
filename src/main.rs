mod connection;
mod mailbox;
mod message;
mod session;

use crate::connection::Connections;
use crate::mailbox::Mailbox;
use std::fs;

// TODO: manage errors
fn main() {
    let data = fs::read_to_string("config.toml").unwrap();

    Connections::new(&data).iter().for_each(|connection| {
        Mailbox::fetch_all(connection);
    });
}
