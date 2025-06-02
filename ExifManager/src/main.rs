mod exif_tool;

use exif_tool::cleaner;
use exif_tool::reader;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "exif_tool-tool")]
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Read { file } => reader::read_metadata(&file)?,
        Commands::Remove { file, output, overwrite } => {
            cleaner::remove_exif(&file, output.as_ref(), overwrite)?
        }
        Commands::Info => {
            println!("=== ExifManager ===");
            println!("Jakub Stachecki, Krystian Bulanda");
            println!("Supported formats for reading EXIF:");
            println!("  • JPEG (.jpg, .jpeg)");
            println!("  • TIFF (.tif, .tiff)");
            println!("  • HEIF (.heif, .heic)");
            println!("  • WebP (.webp)\n");
            println!("Supported for EXIF removal:");
            println!("  • JPEG (.jpg, .jpeg)");
            println!("  • TIFF (.tif, .tiff)\n");
            println!("Libraries used:");
            println!("  • exif (kamadak-exif)");
            println!("  • clap");
            println!("  • anyhow");
            println!("  • img-parts");
            println!("  • image");
        }
    }

    Ok(())
}
