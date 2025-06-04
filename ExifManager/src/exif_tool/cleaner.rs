use anyhow::{Result, bail};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use image::{ ImageReader};
use img_parts::ImageEXIF;
use img_parts::jpeg::Jpeg;
use img_parts::png::Png;
use img_parts::webp::WebP;
use image::ImageFormat;


pub fn remove_exif(file_path: &PathBuf, output_path: Option<&PathBuf>, overwrite: bool) -> Result<()> {
    if !overwrite && output_path.is_none() {
        bail!("Specify --output or use --overwrite");
    }
    let output = output_path.unwrap_or(file_path);

    match file_path.extension().and_then(|s| s.to_str()) {
        Some("jpg") | Some("jpeg") | Some("JPG") | Some("JPEG") => {
            remove_exif_from_jpeg(file_path, output)
        }
        Some("tiff") | Some("TIFF") | Some("TIF") | Some("tif") => {
            remove_exif_from_tiff(file_path, output)
        }
        Some("png") | Some("PNG") => {
            remove_exif_from_png(file_path, output)
        }
        Some("webp") | Some("WEBP") => {
            remove_exif_from_webp(file_path, output)
        }
        Some(ext) => bail!("Unsupported file format: {}", ext),
        None => bail!("Cannot determine file format"),
    }
}

fn remove_exif_from_jpeg(input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    let input_data = fs::read(input_path)?;
    let mut jpeg = Jpeg::from_bytes(input_data.into())?;

    jpeg.set_exif(None);

    let mut out_file = File::create(output_path)?;
    jpeg.encoder().write_to(&mut out_file)?;
    
    println!("EXIF metadata removed; saved to {:?}", output_path);
    Ok(())
}

fn remove_exif_from_png(input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    let input_data = fs::read(input_path)?;
    let mut png = Png::from_bytes(input_data.into())?;
    
    png.set_exif(None);
    
    let mut out_file = File::create(output_path)?;
    png.encoder().write_to(&mut out_file)?;

    println!("EXIF metadata removed from PNG; saved to {:?}", output_path);
    Ok(())
}

fn remove_exif_from_webp(input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    let input_data = fs::read(input_path)?;
    let mut webp = WebP::from_bytes(input_data.into())?;
    
    webp.set_exif(None);
    
    let mut out_file = File::create(output_path)?;
    webp.encoder().write_to(&mut out_file)?;
    
    println!("EXIF metadata removed from WebP; saved to {:?}", output_path);
    Ok(())
}

fn remove_exif_from_tiff(input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    let img = ImageReader::open(input_path)?
        .with_guessed_format()?
        .decode()?;

    let mut out = File::create(output_path)?;
    img.write_to(&mut out, ImageFormat::Tiff)?;

    println!("All metadata stripped; saved to {:?}", output_path);
    Ok(())
}

