use crate::session::ImapSession;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Message {
    pub uid: imap::types::Uid,
    pub body: Vec<u8>,
}

impl Message {
    // TODO: retrieve last saved email to avoid refetching everything
    pub fn fetch(from: u32, to: u32, imap_session: &mut ImapSession) -> Vec<Message> {
        let result = imap_session.fetch(format!("{from}:{to}"), "(UID RFC822)");

        if let Ok(messages) = result {
            return messages
                .iter()
                .map(|message| Message {
                    uid: message.uid.unwrap(),
                    body: message.body().map(|x| x.to_vec()).unwrap(),
                })
                .collect::<Vec<_>>();
        } else {
            println!("{:?}", result);
        }

        vec![]
    }

    pub fn save(dir: &str, message: &Message) {
        let path = format!("{dir}/{:07}.eml", &message.uid);

        if !Path::new(&path).exists() {
            println!("Saving message to {path}");
            fs::write(&path, &message.body).unwrap();
        }
    }
}
