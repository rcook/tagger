use crate::db;
use crate::error::Result;
use crate::item::Item;
use crate::project::Project;

pub fn do_report(project: &Project) -> Result<()> {
    println!("Scanning {}", project.dir.to_str()?);

    let conn = project.open_db_connection()?;
    project
        .create_sample_visitor()
        .visit(&project.dir, &|entry| {
            let p = entry.path();
            println!("Found {}", p.to_str()?);
            let item = Item::from_file(&project.dir, &p)?;
            let item_by_location = db::Item::by_location(&conn, &item.location)?;
            match item_by_location {
                Some(x) => println!(
                    "With same location: {:?} signatures_match={}",
                    x,
                    x.signature.eq(&item.signature)
                ),
                None => println!("Item not found"),
            }
            let item_by_signature = db::Item::by_signature(&conn, &item.signature)?;
            match item_by_signature {
                Some(x) => println!(
                    "With same signature: {:?} locations_match={}",
                    x,
                    x.location.eq(&item.location)
                ),
                None => println!("Item not found"),
            }
            Ok(())
        })?;
    Ok(())
}
