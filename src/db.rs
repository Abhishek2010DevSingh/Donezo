use anyhow::Context;
use rusqlite::Connection;

pub fn conn() -> anyhow::Result<Connection> {
    return Connection::open("donezo.db").context("Error while opening db connection");
}
