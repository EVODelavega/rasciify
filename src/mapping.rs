use crate::types;
pub const ASCII_CHARS: &[char] = &['Ñ', '@', '#', 'W', '$', '9', '8', '7', '6', '5', '4', '3', '2', '1', '0', '?', '!', 'a', 'b', 'c', ';', ':', '+', '=', '-', ',', '.', '_', ' '];

const CHAR_MOD :usize = ASCII_CHARS.len();
//
const CHAR_STEP : f64 = 766.0 / CHAR_MOD as f64;

/// pixel_to_char maps a combined pixel value (RGB channels summed) onto ASCII_CHARS.
/// If the neg flag is true, then the mapping is inverted.
pub fn pixel_to_char(pixel :&u16, neg :bool) -> char {
    let offset = f64::from(*pixel) / CHAR_STEP;
    if neg {
        ASCII_CHARS[CHAR_MOD - ((offset as usize % CHAR_MOD) + 1)]
    } else {
        ASCII_CHARS[offset as usize % CHAR_MOD]
    }
}

pub fn pixel_to_colour_char(pixel : types::Pxl, neg : bool, preview : bool) -> String {
    let char = if preview {
        ' '
    } else {
        pixel_to_char(&pixel.char, neg)
    };
    if preview {
        format!("\x1b[48;2;{};{};{}m{char} \x1b[0m", pixel.colour[0], pixel.colour[1], pixel.colour[2])
    } else {
        format!("\x1b[38;2;{};{};{}m{char} \x1b[0m", pixel.colour[0], pixel.colour[1], pixel.colour[2])
    }
}

