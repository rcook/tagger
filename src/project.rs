use rusqlite::Connection;
use std::path::{Path, PathBuf};

use crate::db::initialize_db;
use crate::error::Result;
use crate::media_path_checker::MediaPathChecker;
use crate::sample_visitor::PathChecker;

pub struct Project {
    pub dir: PathBuf,
    pub db_path: PathBuf,
    pub path_checker: MediaPathChecker,
}

impl Project {
    pub fn from_dir<P: AsRef<Path>>(dir: P) -> Self {
        let db_path = dir.as_ref().join("tagger.db");
        Self {
            dir: dir.as_ref().to_owned(),
            db_path: db_path,
            path_checker: MediaPathChecker::new(),
        }
    }

    pub fn open_db_connection(&self) -> Result<Connection> {
        let conn = Connection::open(&self.db_path)?;
        rusqlite::vtab::array::load_module(&conn)?;
        initialize_db(&conn)?;
        Ok(conn)
    }

    pub fn path_checker(&self) -> &impl PathChecker {
        &self.path_checker
    }
}
