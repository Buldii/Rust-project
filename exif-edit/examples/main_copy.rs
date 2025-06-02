use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Cursor, Write};
use std::path::Path;
use exif::Reader;
use img_parts::{jpeg::{Jpeg, markers}};
use img_parts::ImageEXIF;

fn main() {
    let path = Path::new("C:/Users/Kuba/Downloads/Kodak_CX7530.jpg");
    // remove_exif("C:/Users/Kuba/Downloads/Kodak_CX7530.jpg".as_ref(), "C:/Users/Kuba/Downloads/Kodak_CX7530.jpg".as_ref()).expect("failed");
    match extract_exif_tags(path) {
        Ok(tags) => {
            println!("EXIF tagi (tag, wartość):");
            for (tag, value) in tags {
                println!("{}: {}", tag, value);
            }
        }
        Err(e) => eprintln!("Błąd: {}", e),
    }
}

fn extract_exif_tags(path: &Path) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;


    let jpeg = Jpeg::from_bytes(data.into())?;

    for segment in jpeg.segments() {
        if segment.marker() == markers::APP1 {
            let content = segment.contents();

            if content.starts_with(b"Exif\0\0") {
                let exif_data = &content[6..];
                return parse_exif(exif_data);
            } else {
                return parse_exif(content);
            }

        }

    }

    Err("Brak danych EXIF w pliku".into())
}

fn parse_exif(exif_data: &[u8]) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(exif_data);
    let reader = Reader::new();
    let exif = reader.read_from_container(&mut cursor)?;

    let mut tags = Vec::new();

    for field in exif.fields() {
        tags.push((
            field.tag.to_string(),
            field.display_value().with_unit(&exif).to_string(),
        ));
    }
    Ok(tags)
}

fn remove_exif(input: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let input_data = fs::read(input)?;
    let mut jpeg = Jpeg::from_bytes(input_data.into())?;
    
    jpeg.set_exif(None);
    
    let mut out_file = File::create(output)?;
    jpeg.encoder().write_to(&mut out_file)?;

    Ok(())
}


