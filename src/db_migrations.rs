use rusqlite::{params, Connection, NO_PARAMS};
use std::collections::HashSet;

use crate::error::Result;

// Migrations will be run in the order defined in this array
static MIGRATIONS: &'static [(fn(&Connection) -> Result<()>, &'static str)] = &[
    (do_migration_202103210001, "202103210001"),
    (do_migration_202103210002, "202103210002"),
];

fn do_initial_migration(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS migrations (
            name        TEXT NOT NULL PRIMARY KEY
        );",
    )?;
    Ok(())
}

fn do_migration_202103210001(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE items (
            id          INTEGER PRIMARY KEY,
            location    TEXT NOT NULL UNIQUE,
            signature   TEXT NOT NULL UNIQUE
        );
        CREATE TABLE duplicate_items (
            id          INTEGER PRIMARY KEY,
            location    TEXT NOT NULL UNIQUE,
            signature   TEXT NOT NULL
        );
        CREATE TABLE tags (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL UNIQUE
        );
        CREATE TABLE item_tags (
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

fn do_migration_202103210002(conn: &Connection) -> Result<()> {
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

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "PRAGMA foreign_keys = ON;
        PRAGMA foreign_key_check;",
    )?;

    do_initial_migration(conn)?;

    let mut stmt = conn.prepare("SELECT name FROM migrations")?;
    let names = stmt
        .query_map(NO_PARAMS, |row| Ok(row.get::<_, String>(0)?))?
        .collect::<rusqlite::Result<HashSet<_>>>()?;

    for m in MIGRATIONS {
        if !names.contains(m.1) {
            m.0(conn)?;
            let mut stmt = conn.prepare("INSERT INTO migrations (name) VALUES (?1)")?;
            stmt.execute(params![m.1])?;
        }
    }

    Ok(())
}
