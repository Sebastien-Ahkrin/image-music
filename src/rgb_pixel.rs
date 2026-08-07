use crate::hsl_pixel::HslPixel;
use std::fmt::{Display, Formatter};

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
        let normalized_red = self.r as f32 / 255.0;
        let normalized_green = self.g as f32 / 255.0;
        let normalized_blue = self.b as f32 / 255.0;

        let maximum_value = normalized_red.max(normalized_green).max(normalized_blue);
        let minimum_value = normalized_red.min(normalized_green).min(normalized_blue);

        let lightness = (maximum_value + minimum_value) / 2.0;

        if maximum_value == minimum_value {
            return HslPixel::new(0.0, 0.0, lightness);
        }

        let delta = maximum_value - minimum_value;

        let saturation = match lightness > 0.5 {
            true => delta / (2.0 - maximum_value - minimum_value),
            false => delta / (maximum_value + minimum_value),
        };

        let mut hue: f32;
        if maximum_value == normalized_red {
            hue = (normalized_green - normalized_blue) / delta
                + (match normalized_green < normalized_blue {
                    true => 6.0,
                    false => 0.0,
                });
        } else if maximum_value == normalized_green {
            hue = (normalized_blue - normalized_red) / delta + 2.0;
        } else {
            hue = (normalized_red - normalized_green) / delta + 4.0;
        }

        hue = hue * 60.0;

        HslPixel::new(hue, saturation, lightness)
    }
}

#[cfg(test)]
mod tests {
    use crate::rgb_pixel::RgbPixel;
    use test_case::test_case;

    #[test_case(255, 0, 0, 0.0, 1.0, 0.5 ; "rouge")]
    #[test_case(0, 255, 0, 120.0, 1.0, 0.5 ; "vert")]
    #[test_case(0, 0, 255, 240.0, 1.0, 0.5 ; "bleu")]
    #[test_case(255, 255, 255, 0.0, 0.0, 1.0 ; "blanc")]
    #[test_case(0, 0, 0, 0.0, 0.0, 0.0 ; "noir")]
    #[test_case(128, 128, 128, 0.0, 0.0, 0.5019608 ; "gris")]
    fn converts_rgb_to_hsl(
        r: u8,
        g: u8,
        b: u8,
        expected_hue: f32,
        expected_sat: f32,
        expected_light: f32,
    ) {
        let pixel = RgbPixel { r, g, b };
        let hsl = pixel.to_hsl_pixel();

        assert!((hsl.hue - expected_hue).abs() < 0.01);
        assert!((hsl.saturation - expected_sat).abs() < 0.01);
        assert!((hsl.lightness - expected_light).abs() < 0.01);
    }
}
