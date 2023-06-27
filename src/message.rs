use crate::session::Session;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Message {
    pub uid: imap::types::Uid,
    pub body: Vec<u8>,
}

impl Message {
    // TODO: retrieve last saved email to avoid refetching everything
    pub fn fetch(from: u32, to: u32, session: &mut Session) -> Result<Vec<Message>> {
        let messages = session.get_messages_by_range(from, to)?;

        Ok(messages
            .iter()
            .map(|message| Message {
                uid: message.uid.unwrap(),
                body: message.body().map(<[u8]>::to_vec).unwrap(),
            })
            .collect::<Vec<_>>())
    }

    pub fn save(dir: &str, message: &Message) -> Result<()> {
        let path = format!("{dir}/{:07}.eml", &message.uid);

        if !Path::new(&path).exists() {
            println!("Saving message to {path}");
            fs::write(&path, &message.body).context("Unable to write message to file")?;
        }

        Ok(())
    }
}
