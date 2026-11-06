use image::ImageReader;
use crate::types::{
    Pxl,
    Scale,
};
use std::fs;

/// returns the RGB values per pixel as a sum (R + G + B) to map onto ascii chars.
pub fn get_pixel_values_from_path(path: &str, scale: &mut Scale) -> anyhow::Result<(u32, Vec<u16>)> {
    fs::exists(path).expect("Can't read image file");
    let img = ImageReader::open(path)?.decode()?;
    scale.calculate_factor(img.width(), img.height());
    let (w, h) = scale.get_resize_params(img.width(), img.height());
    // Nearest will do for now
    let packed = img.resize(w, h,scale.filter).into_rgb8();
    let combined: Vec<u16> = packed
        .pixels()
        .map(|p| u16::from(p.0[0]) + u16::from(p.0[1]) + u16::from(p.0[2]))
        .collect();
    Ok((w, combined))
}

pub fn get_pixels_from_path(path: &str, scale: &mut Scale) -> anyhow::Result<(u32, Vec<Pxl>)> {
    fs::exists(path).expect("Can't read image file");
    let img = ImageReader::open(path)?.decode()?;
    scale.calculate_factor(img.width(), img.height());
    let (w, h) = scale.get_resize_params(img.width(), img.height());
    // detailed scaling because this is used for preview
    let packed = img.resize(w, h, scale.filter).into_rgb8();
    let pxls: Vec<Pxl> = packed.pixels()
        .map(|p| Pxl{
            colour: *p,
            char: u16::from(p.0[0]) + u16::from(p.0[1]) + u16::from(p.0[2]),
        }).collect();
    Ok((w, pxls))
}

