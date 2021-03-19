use rusqlite::Connection;
use std::path::{Path, PathBuf};

use crate::db::create_schema;
use crate::error::Result;
use crate::extension_set::ExtensionSet;
use crate::sample_visitor::SampleVisitor;

pub struct Project {
    pub dir: PathBuf,
    pub db_path: PathBuf,
}

impl Project {
    const MEDIA_FILE_EXTENSIONS: [&'static str; 8] =
        ["aiff", "au", "mid", "m4a", "mp3", "snd", "wav", "wma"];

    pub fn from_dir<P: AsRef<Path>>(dir: P) -> Self {
        let db_path = dir.as_ref().join("tagger.db");
        Self {
            dir: dir.as_ref().to_owned(),
            db_path: db_path,
        }
    }

    pub fn open_db_connection(&self) -> Result<Connection> {
        let conn = Connection::open(&self.db_path)?;
        rusqlite::vtab::array::load_module(&conn)?;
        create_schema(&conn)?;
        Ok(conn)
    }

    pub fn create_sample_visitor(&self) -> SampleVisitor {
        SampleVisitor::new(ExtensionSet::new(&Self::MEDIA_FILE_EXTENSIONS))
    }
}
