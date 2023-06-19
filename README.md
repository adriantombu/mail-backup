# Make a backup of your emails

This project allows you to make a local copy of your emails by populating their credentials into a `config.toml` file.

Everything is read only, so this script will never do any destructive action on your emails.

## Getting started

### Add a new connection

To add a new connection append the following values to your config.toml file:

```toml
[[connections]]
domain = "imap.example.com"
username = "john.do@example.com"
password = "mysecretpassword"
```

### How yo use

- Clone the repository `git@github.com:adriantombu/mail-backup.git`
- Create and populate the `config.toml` file in the cloned directory
- Run `cargo run -r` (it may take some time if you have tens of thousands of emails)
- The export can be found in the `export/` directory
