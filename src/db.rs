use anyhow::{Context, Result};
use rusqlite::{params, Connection};

pub fn conn() -> Result<Connection> {
    Connection::open("donezo.db").context("Failed to open database connection")
}

pub fn migration(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL UNIQUE,
            done BOOLEAN DEFAULT FALSE,
            due_time DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
        params![],
    )
    .context("Failed to run database migration")?;

    Ok(())
}
