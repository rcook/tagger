mod db;
mod walk;

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;

use crate::db::do_db;
use crate::walk::{ExtensionSet, SampleVisitor};

fn do_walk() -> io::Result<()> {
    let visitor = SampleVisitor::new(ExtensionSet::new(&["aiff", "wav"]));

    let home_dir = dirs::home_dir().expect("could not determine home directory");

    println!(
        "Scanning {}",
        home_dir.to_str().expect("could not convert path")
    );
    visitor.visit(&home_dir, &|entry| {
        let p = entry.path();
        let mut file = File::open(&p)?;
        let mut hasher = Sha256::new();
        io::copy(&mut file, &mut hasher)?;
        let hash = hasher.finalize();
        println!(
            "{}: {:x}",
            p.to_str().expect("could not convert path"),
            hash
        );
        Ok(())
    })?;

    Ok(())
}

fn main() -> io::Result<()> {
    //do_walk()
    do_db().expect("database test failed");
    Ok(())
}
