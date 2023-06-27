use crate::connection::Connection;
use anyhow::{Context, Result};
use imap::types::{Fetch, Mailbox, Name, ZeroCopy};

pub type Imap = imap::Session<native_tls::TlsStream<std::net::TcpStream>>;

pub struct Session(Imap);

impl Session {
    pub fn try_new(connection: &Connection) -> Result<Self> {
        let Connection {
            domain,
            username,
            password,
        } = connection;

        let tls = native_tls::TlsConnector::builder()
            .build()
            .context("Could not build tls connexion")?;
        let client = imap::connect((domain.as_str(), 993), domain.as_str(), &tls)
            .context("Could not connect to IMAP server")?;

        Ok(Self(client.login(username, password).unwrap()))
    }

    pub fn list_mailboxes(&mut self) -> Result<ZeroCopy<Vec<Name>>> {
        self.0
            .list(Some("*"), Some("*"))
            .context("Could not list mailboxes")
    }

    pub fn get_mailbox(&mut self, mailbox_name: &str) -> Result<Mailbox> {
        self.0
            .examine(mailbox_name)
            .with_context(|| format!("Could not read mailbox {mailbox_name}"))
    }

    pub fn get_messages_by_range(&mut self, from: u32, to: u32) -> Result<ZeroCopy<Vec<Fetch>>> {
        self.0
            .fetch(format!("{from}:{to}"), "(UID RFC822)")
            .with_context(|| format!("Could not retrieve messages with range {from}:{to}"))
    }

    pub fn close(&mut self) -> Result<()> {
        self.0.close().context("Could not close IMAP connection")
    }
}
