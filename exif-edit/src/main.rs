use std::env::args;
use std::fs;
use std::path::Path;
use std::process::exit;

use bytes::Bytes;
use img_parts::{DynImage, ImageEXIF};

fn main() {
    let mut args = args();
    args.next();

    let input_path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("Please specify the input file path (must be a jpeg, png or webp)");
            exit(1);
        }
    };

    println!("Loading the image and extracting EXIF data...");
    let exif = load_exif(input_path.as_ref());

    match exif {
        Some(exif_data) => {
            println!("EXIF data found!");
            println!("EXIF data size: {} bytes", exif_data.len());
            println!("EXIF data (hex): {}", hex::encode(&exif_data));

            // Próba wyświetlenia czytelnych danych EXIF
            display_exif_info(&exif_data);
        }
        None => {
            println!("No EXIF data found in the image.");
        }
    }
}

/// Ładuje obraz i ekstraktuje dane EXIF
fn load_exif(path: &Path) -> Option<Bytes> {
    // Czytaj plik wejściowy
    let buf = fs::read(path).expect("Failed to read input file");

    // Ekstraktuj dane EXIF używając img-parts
    DynImage::from_bytes(buf.into())
        .expect("Failed to load image")
        .and_then(|dimg| dimg.exif())
}

/// Wyświetla podstawowe informacje z danych EXIF
fn display_exif_info(exif_data: &[u8]) {
    println!("\n--- EXIF Information ---");

    // Sprawdź czy dane zaczynają się od poprawnego nagłówka EXIF
    if exif_data.len() >= 6 {
        let header = &exif_data[0..6];
        if header == b"Exif\0\0" {
            println!("Valid EXIF header found");

            // Wyświetl pierwsze kilkadziesiąt bajtów jako podgląd
            println!("First 32 bytes (after header):");
            let preview_end = std::cmp::min(exif_data.len(), 38);
            let preview = &exif_data[6..preview_end];

            for (i, byte) in preview.iter().enumerate() {
                if i % 16 == 0 && i > 0 {
                    println!();
                }
                print!("{:02x} ", byte);
            }
            println!();
        } else {
            println!("EXIF header not found or invalid");
        }
    }

    // Sprawdź endianness (byte order)
    if exif_data.len() >= 10 {
        let tiff_header = &exif_data[6..10];
        match tiff_header {
            b"II*\0" => println!("Byte order: Little-endian (Intel)"),
            b"MM\0*" => println!("Byte order: Big-endian (Motorola)"),
            _ => println!("Unknown byte order"),
        }
    }
}

