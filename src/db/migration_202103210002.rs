use rusqlite::Connection;

use crate::error::Result;

pub fn run_migration(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "PRAGMA foreign_keys = OFF;
        BEGIN TRANSACTION;
        CREATE TABLE _new_item_tags (
            id          INTEGER PRIMARY KEY,
            item_id     INTEGER NOT NULL,
            tag_id      INTEGER NOT NULL,
            FOREIGN KEY(item_id) REFERENCES items(id) ON DELETE CASCADE,
            FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE,
            UNIQUE(item_id, tag_id)
        );
        INSERT INTO _new_item_tags SELECT * FROM item_tags;
        DROP TABLE item_tags;
        ALTER TABLE _new_item_tags RENAME TO item_tags;
        PRAGMA foreign_key_check;
        COMMIT;
        PRAGMA foreign_keys = ON;",
    )?;
    Ok(())
}
