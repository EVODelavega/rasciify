mod mapping;
mod reader;
mod writer;
mod types;

use clap::Parser;
use anyhow::Result;
use types::{Cli, Scale};

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut out = writer::Out::new(args.output.as_str())?;
    let mut scale = Scale::parse(&args);
    if !args.colour {
    let (width, pixels) = reader::get_pixel_values_from_path(args.image.as_str(), &mut scale)?;
        for row in pixels.chunks(width as usize) {
            let line: String = row.iter()
                .flat_map(|p| [ mapping::pixel_to_char(p, args.negative), ' '])
                .collect();
            out.write_ln(line)?;
        }
    } else {
        let (width, pixels) = reader::get_pixels_from_path(args.image.as_str(), &mut scale)?;
        for row in pixels.chunks(width as usize) {
            let line: Vec<String> = row.iter()
                .map(|p| mapping::pixel_to_colour_char(&mut p.clone(), args.negative, args.preview))
                .collect();
            out.write_ln(line.concat())?;
        }
    }
    if out.is_file() {
        println!("Written {}", args.output.as_str());
    }
    Ok(())
}

