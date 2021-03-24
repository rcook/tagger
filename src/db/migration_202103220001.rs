use rusqlite::Connection;

use crate::result::Result;

pub fn run_migration(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "PRAGMA foreign_keys = OFF;
        BEGIN TRANSACTION;

        CREATE TABLE files (
            id          INTEGER PRIMARY KEY,
            location    TEXT NOT NULL UNIQUE,
            signature   TEXT NOT NULL UNIQUE
        );
        INSERT INTO files SELECT * FROM items;

        CREATE TABLE duplicate_files (
            id          INTEGER PRIMARY KEY,
            location    TEXT NOT NULL UNIQUE,
            signature   TEXT NOT NULL
        );
        INSERT INTO duplicate_files SELECT * FROM duplicate_items;

        CREATE TABLE file_tags (
            id          INTEGER PRIMARY KEY,
            file_id     INTEGER NOT NULL,
            tag_id      INTEGER NOT NULL,
            FOREIGN KEY(file_id) REFERENCES files(id),
            FOREIGN KEY(tag_id) REFERENCES tags(id),
            UNIQUE(file_id, tag_id)
        );
        INSERT INTO file_tags SELECT * FROM item_tags;

        DROP TABLE item_tags;
        DROP TABLE items;
        DROP TABLE duplicate_items;

        PRAGMA foreign_key_check;
        COMMIT;
        PRAGMA foreign_keys = ON;",
    )?;
    Ok(())
}
