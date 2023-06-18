use serde::Deserialize;
use std::fs;
use std::path::Path;

type ImapSession = imap::Session<native_tls::TlsStream<std::net::TcpStream>>;

#[derive(Debug)]
struct Message {
    uid: imap::types::Uid,
    body: Vec<u8>,
}

#[derive(Deserialize, Debug)]
struct Connections {
    connections: Vec<Connection>,
}

#[derive(Deserialize, Debug)]
struct Connection {
    // TODO: &str
    domain: String,
    username: String,
    password: String,
}

fn main() {
    // dotenv::dotenv().unwrap();
    let data = fs::read_to_string("config.toml").unwrap();
    let connections = toml::from_str::<Connections>(data.as_str())
        .unwrap()
        .connections;

    for connection in connections {
        fetch_inbox(connection).unwrap();
    }
}

fn fetch_inbox(connection: Connection) -> imap::error::Result<Option<String>> {
    let Connection {
        domain,
        username,
        password,
    } = connection;

    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain.as_str(), 993), domain.as_str(), &tls).unwrap();

    let mut imap_session = client.login(&username, password).unwrap();

    // List all folders & messages
    let folders = imap_session.list(Some("*"), Some("*")).unwrap();
    folders.iter().for_each(|f| {
        let dir = format!("export/{username}/{}", f.name());
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
