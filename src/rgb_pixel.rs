use crate::hsl_pixel::HslPixel;
use std::fmt::{Display, Formatter};
use palette::{Hsl, Srgb, FromColor, IntoColor, Lch};

pub struct RgbPixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Display for RgbPixel {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!("r:{}, g:{}, b:{}", self.r, self.g, self.b))
    }
}

impl RgbPixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hsl_pixel(&self) -> HslPixel {
        let rgb = Srgb::new(self.r, self.g, self.b).into_format::<f32>();
        let hsl: Hsl = rgb.into_color();

        HslPixel::new(hsl.hue.into_positive_degrees(), hsl.saturation, hsl.lightness)
    }
}
