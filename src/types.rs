use clap::{ValueEnum, Parser};
use image::imageops::FilterType;
use image::Rgb;

#[derive(Clone, Debug)]
pub struct Pxl {
    pub colour :Rgb<u8>,
    pub char :u16,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, default_value_t = String::from("examples/vim.png"))]
    pub image: String,
    #[arg(long, short, default_value_t = 1.0)]
    scale: f32,
    #[arg(long, short, default_value_t = 0)]
    width: usize,
    #[arg(long, default_value_t = 0)]
    height: usize,
    #[arg(long, short, default_value_t = String::from("stdout"))]
    pub output: String,
    #[arg(long, short, default_value_t = false)]
    pub negative: bool,
    #[arg(long, short, default_value_t = false)]
    pub colour: bool,
    #[arg(long, short, default_value_t = false)]
    pub preview: bool,
    #[arg(long, short, value_enum, default_value_t = Filter::Nearest)]
    filter: Filter,
}

#[derive(Copy, Clone, Debug)]
pub struct Scale {
    pub filter : FilterType,
    factor: f32,
    width: usize,
    height: usize,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Filter {
    /// Fastest, blocky
    Nearest,
    /// Good enough usually
    Triangle,
    /// Cubic
    CatmullRom,
    /// Super slow, not really useful
    Gaussian,
    /// Slowest, for completeness, but don't use.
    Lanczos3,
}

impl From<Filter> for FilterType {
    fn from(f: Filter) -> Self {
        match f {
            Filter::Nearest => FilterType::Nearest,
            Filter::Triangle => FilterType::Triangle,
            Filter::CatmullRom => FilterType::CatmullRom,
            Filter::Gaussian => FilterType::Gaussian,
            Filter::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

impl Pxl {
    pub fn invert_colour(&mut self, neg: bool) {
        if neg {
            self.colour[0] ^= 255;
            self.colour[1] ^= 255;
            self.colour[2] ^= 255;
        }
    }
}
 
impl Scale {
    pub fn parse(args: &Cli) -> Scale {
        Scale{
            filter: args.filter.into(),
            factor: args.scale,
            width: args.width,
            height: args.height,
        }
    }

    pub fn get_resize_params(self, raw_width: u32, raw_height: u32) -> (u32, u32) {
        let ratio = (raw_width as f32 / raw_height as f32) * self.factor;
        ((raw_width as f32 * ratio).round() as u32, (raw_height as f32 * ratio).round() as u32)
    }

    pub fn calculate_factor(&mut self, raw_width: u32, raw_height: u32) {
        if self.factor != 1.0 || (self.width == 0 && self.height == 0 ) {
            return
        }
        if self.width != 0 {
            self.factor = 1.0 / (raw_width as f32 / self.width as f32);
        } else {
            self.factor = 1.0 / (raw_height as f32 / self.height as f32);
        }
    }
}

