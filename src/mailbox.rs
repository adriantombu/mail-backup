use crate::connection::Connection;
use crate::message::Message;
use crate::session::Session;
use std::fs;

pub struct Mailbox;

impl Mailbox {
    pub fn fetch_all(connection: &Connection) {
        let mut imap_session = Session::new(connection);

        // List all folders & messages
        let folders = imap_session.list(Some("*"), Some("*")).unwrap();
        folders.iter().for_each(|f| {
            let dir = format!("export/{}/{}", &connection.username, f.name());
            println!("--- {:?}", f);

            let mailbox = imap_session.examine(f.name()).unwrap();
            println!("{:?}", mailbox);

            if mailbox.exists > 0 {
                fs::create_dir_all(&dir).unwrap();

                for message in Message::fetch(1, mailbox.exists, &mut imap_session).into_iter() {
                    Message::save(&dir, &message);
                }
            } else {
                println!("No messages found, skipping...");
            }
        });

        imap_session.close().unwrap()
    }
}
