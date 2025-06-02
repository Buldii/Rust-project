use img_parts::jpeg::{Jpeg, markers};
use std::fs::File;
use std::io::{Read, Cursor};
use std::path::Path;
use exif::Reader;

fn main() {
    let path = Path::new("C:/Users/Kuba/Downloads/Canon_PowerShot_S40.png"); 

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
                // Pomijamy nagłówek b"Exif\0\0", który ma długość 6 bajtów
                let exif_data = &content[6..];
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
                return Ok(tags);
            }
        }
    }

    Err("Brak danych EXIF w pliku".into())
}



