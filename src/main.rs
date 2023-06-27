mod connection;
mod mailbox;
mod message;
mod session;

use crate::connection::Connections;
use crate::mailbox::Mailbox;
use anyhow::{Context, Result};
use std::fs;

fn main() -> Result<()> {
    let data = fs::read_to_string("config.toml").context("Could not read config.toml file")?;

    for connection in &Connections::try_new(&data)?.connections {
        Mailbox::fetch_all(connection)?;
    }

    Ok(())
}
