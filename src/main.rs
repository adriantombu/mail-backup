use std::path::Path;
use std::{env, fs};

type ImapSession = imap::Session<native_tls::TlsStream<std::net::TcpStream>>;

#[derive(Debug)]
struct Message {
    uid: imap::types::Uid,
    body: Vec<u8>,
}

fn main() {
    dotenv::dotenv().unwrap();

    // To connect to the gmail IMAP server with this you will need to allow unsecure apps access.
    // See: https://support.google.com/accounts/answer/6010255?hl=en
    // Look at the gmail_oauth2.rs example on how to connect to a gmail server securely.
    fetch_inbox_top().unwrap();
}

fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let domain = &*Box::leak(env::var("IMAP_SERVER").unwrap().into_boxed_str());
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    let mut imap_session = client
        .login(
            env::var("IMAP_MAIL").unwrap(),
            env::var("IMAP_PASSWORD").unwrap(),
        )
        .unwrap();

    // List all folders & messages
    let folders = imap_session.list(Some("*"), Some("*")).unwrap();
    folders.iter().for_each(|f| {
        let dir = format!("export/{}", f.name());
        println!("--- {:?}", f);

        let mailbox = imap_session.examine(f.name()).unwrap();
        println!("{:?}", mailbox);

        if mailbox.exists > 0 {
            fs::create_dir_all(&dir).unwrap();

            for message in messages(1, mailbox.exists, &mut imap_session).into_iter() {
                message_store(&dir, &message);
            }
        } else {
            println!("No messages found, skipping...");
        }
    });

    Ok(Some("".to_string()))
}

// TODO: retrieve last saved email to avoid refetching everything
fn messages(from: u32, to: u32, imap_session: &mut ImapSession) -> Vec<Message> {
    let result = imap_session.fetch(format!("{}:{}", from, to), "(UID RFC822)");

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

fn message_store(dir: &str, message: &Message) {
    let path = format!("{dir}/{:07}.eml", &message.uid);

    if !Path::new(&path).exists() {
        println!("Saving message to {path}");
        fs::write(&path, &message.body).unwrap();
    }
}
