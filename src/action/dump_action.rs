use crate::db;
use crate::error::Result;
use crate::project::Project;

pub fn do_dump(project: &Project) -> Result<()> {
    let conn = project.open_db_connection()?;

    println!("Project directory: {}", project.dir.display());
    println!("Database path: {}", project.db_path.display());

    println!("Items:");
    for item in db::Item::all(&conn)? {
        println!("  ({}): {}, {}", item.id, item.location, item.signature);
    }

    println!("Tags:");
    for tag in db::Tag::all(&conn)? {
        println!("  ({}): {}", tag.id, tag.name);
    }

    println!("Item tags:");
    for item_tag in db::ItemTag::all(&conn)? {
        println!(
            "  ({}): {}, {}",
            item_tag.id, item_tag.item_id, item_tag.tag_id
        );
    }

    Ok(())
}
