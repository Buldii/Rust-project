use anyhow::{Result};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn read_metadata(file_path: &PathBuf) -> Result<()> {
    let file = File::open(file_path)?;
    let mut bufreader = BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    
    let exif = exif_reader.read_from_container(&mut bufreader)?;
    
    println!("=== EXIF metadata for {:?} ===", file_path.file_name().unwrap());
    if exif.fields().count() == 0 {
        println!("No EXIF metadata found.");
    } else {
        display_all_fields(&exif);
        println!("\nTotal EXIF fields: {}", exif.fields().count());
    }
    Ok(())
}

fn display_all_fields(exif: &exif::Exif) {
    let mut fields_list: Vec<_> = exif.fields().collect();

    fields_list.sort_by_key(|field| field.tag.to_string());
    
    for field in fields_list {
        println!(
            "{}: {}",
            field.tag,
            field.display_value().with_unit(exif)
        );
    }
}
