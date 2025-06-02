use std::fs;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use img_parts::ImageEXIF;
use img_parts::jpeg::Jpeg;

#[derive(Parser)]
#[command(name = "exif-tool")]
#[command(about = "A simple EXIF metadata manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Read {
        #[arg(short, long)]
        file: PathBuf,
    },
    Remove {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long)]
        overwrite: bool,
    },
    Info,
}

struct ExifTool;

impl ExifTool {
    pub fn read_metadata(file_path: &PathBuf) -> Result<()> {
        let file = File::open(file_path)
            .with_context(|| format!("Cannot open file: {:?}", file_path))?;
        let mut bufreader = BufReader::new(&file);
        let exifreader = exif::Reader::new();

        match exifreader.read_from_container(&mut bufreader) {
            Ok(exif) => {
                println!("=== EXIF metadata for {:?} ===", file_path.file_name().unwrap());
                if exif.fields().count() == 0 {
                    println!("No EXIF metadata found.");
                } else {
                    Self::display_all_fields(&exif);
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

    pub fn remove_exif(file_path: &PathBuf, output_path: Option<&PathBuf>, overwrite: bool) -> Result<()> {
        if !overwrite && output_path.is_none() {
            anyhow::bail!("Specify --output or use --overwrite");
        }
        let output = output_path.unwrap_or(file_path);

        match file_path.extension().and_then(|s| s.to_str()) {
            Some("jpg") | Some("jpeg") | Some("JPG") | Some("JPEG") => {
                Self::remove_exif_from_jpeg(file_path, output)
            }
            Some(ext) => anyhow::bail!("Unsupported file format: {}", ext),
            None => anyhow::bail!("Cannot determine file format"),
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

    pub fn show_info() {
        println!("=== ExifTool ===");
        println!("Version: 0.1.0\n");
        println!("Supported formats for reading EXIF:");
        println!("  • JPEG (.jpg, .jpeg)");
        println!("  • TIFF (.tif, .tiff)");
        println!("  • HEIF (.heif, .heic)");
        println!("  • PNG (.png)");
        println!("  • WebP (.webp)\n");
        println!("Supported for EXIF removal:");
        println!("  • JPEG (.jpg, .jpeg)\n");
        println!("Libraries used:");
        println!("  • exif (kamadak-exif)");
        println!("  • clap");
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Read { file } => ExifTool::read_metadata(&file)?,
        Commands::Remove { file, output, overwrite } => {
            ExifTool::remove_exif(&file, output.as_ref(), overwrite)?
        }
        Commands::Info => ExifTool::show_info(),
    }

    Ok(())
}
