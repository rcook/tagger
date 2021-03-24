use rusqlite::{params, Connection, NO_PARAMS};
use std::collections::HashSet;

use super::migration_202103210001;
use super::migration_202103210002;
use super::migration_202103220001;
use crate::result::Result;

// Migrations will be run in the order defined in this array
static MIGRATIONS: &'static [(fn(&Connection) -> Result<()>, &'static str)] = &[
    (migration_202103210001::run_migration, "202103210001"),
    (migration_202103210002::run_migration, "202103210002"),
    (migration_202103220001::run_migration, "202103220001"),
];

fn do_initial_migration(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS migrations (
            name        TEXT NOT NULL PRIMARY KEY
        );",
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
