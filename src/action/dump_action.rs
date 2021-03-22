use crate::db;
use crate::error::Result;
use crate::project::Project;

pub fn do_dump(project: &Project) -> Result<()> {
    let conn = project.open_db_connection()?;

    println!("Project directory: {}", project.dir.display());
    println!("Database path: {}", project.db_path.display());

    println!("Files:");
    for file in db::File::all(&conn, None)? {
        println!(
            "  ({}): {}, {}",
            file.id,
            file.location.as_str(),
            file.signature.as_str()
        );
    }

    println!("Duplicate items:");
    for file in db::DuplicateFile::all(&conn)? {
        println!(
            "  ({}): {}, {}",
            file.id,
            file.location.as_str(),
            file.signature.as_str()
        );
    }

    println!("Tags:");
    for tag in db::Tag::all(&conn, None)? {
        println!("  ({}): {}", tag.id, tag.name);
    }

    println!("File tags:");
    for file_tag in db::FileTag::all(&conn)? {
        println!(
            "  ({}): {}, {}",
            file_tag.id, file_tag.file_id, file_tag.tag_id
        );
    }

    Ok(())
}
