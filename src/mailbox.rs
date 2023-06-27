use crate::connection::Connection;
use crate::message::Message;
use crate::session::Session;
use anyhow::{Context, Result};
use std::fs;

pub struct Mailbox;

impl Mailbox {
    pub fn fetch_all(connection: &Connection) -> Result<()> {
        let mut imap_session =
            Session::try_new(connection).context("Unable to open an IMAP session")?;

        for mailbox in imap_session.list_mailboxes()?.iter() {
            let dir = format!("export/{}/{}", &connection.username, mailbox.name());
            // TODO: handle special characters in mailbox name
            let mailbox = imap_session.get_mailbox(mailbox.name())?;

            if mailbox.exists > 0 {
                fs::create_dir_all(&dir).context("Could not create directory")?;

                for message in Message::fetch(1, mailbox.exists, &mut imap_session)? {
                    Message::save(&dir, &message)?;
                }
            } else {
                println!("No messages found, skipping...");
            }
        }

        imap_session.close()
    }
}
