mod connection;
mod mailbox;
mod message;
mod session;

use crate::connection::Connections;
use crate::mailbox::Mailbox;
use anyhow::{Context, Result};
use rayon::prelude::*;
use std::fs;

fn main() -> Result<()> {
    let data = fs::read_to_string("config.toml").context("Could not read config.toml file")?;
    let connections = &Connections::try_new(&data)?.connections;

    connections.par_iter().try_for_each(Mailbox::fetch_all)
}
