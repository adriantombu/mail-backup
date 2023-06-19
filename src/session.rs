use crate::connection::Connection;

pub type ImapSession = imap::Session<native_tls::TlsStream<std::net::TcpStream>>;

pub struct Session;

impl Session {
    pub fn new(connection: &Connection) -> ImapSession {
        let Connection {
            domain,
            username,
            password,
        } = connection;

        let tls = native_tls::TlsConnector::builder().build().unwrap();
        let client = imap::connect((domain.as_str(), 993), domain.as_str(), &tls).unwrap();

        client.login(username, password).unwrap()
    }
}
