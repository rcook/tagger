use rusqlite::Connection;

use crate::error::Result;

pub fn run_migration(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS items (
            id          INTEGER PRIMARY KEY,
            location    TEXT NOT NULL UNIQUE,
            signature   TEXT NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS duplicate_items (
            id          INTEGER PRIMARY KEY,
            location    TEXT NOT NULL UNIQUE,
            signature   TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS tags (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS item_tags (
            id          INTEGER PRIMARY KEY,
            item_id     INTEGER NOT NULL,
            tag_id      INTEGER NOT NULL,
            FOREIGN KEY(item_id) REFERENCES items(id),
            FOREIGN KEY(tag_id) REFERENCES tags(id),
            UNIQUE(item_id, tag_id)
        );",
    )?;
    Ok(())
}
