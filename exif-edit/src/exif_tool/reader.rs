use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn read_metadata(file_path: &PathBuf) -> Result<()> {
    let file = File::open(file_path)
        .with_context(|| format!("Cannot open file: {:?}", file_path))?;
    let mut bufreader = BufReader::new(&file);
    let exif_reader = exif::Reader::new();

    match exif_reader.read_from_container(&mut bufreader) {
        Ok(exif) => {
            println!("=== EXIF metadata for {:?} ===", file_path.file_name().unwrap());
            if exif.fields().count() == 0 {
                println!("No EXIF metadata found.");
            } else {
                display_all_fields(&exif);
                println!("\nTotal EXIF fields: {}", exif.fields().count());
            }
        }
        Err(e) => {
            println!("Error reading EXIF metadata: {}", e);
        }
    }
    Ok(())
}

fn display_all_fields(exif: &exif::Exif) {
    for field in exif.fields() {
        println!(
            "{}: {}",
            field.tag,
            field.display_value().with_unit(exif)
        );
    }
}
