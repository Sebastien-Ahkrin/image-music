use crate::rgb_pixel::RgbPixel;
use image::ImageReader;
use std::fmt::{Display, Formatter, Result};

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Display for Image {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_fmt(format_args!("w:{} h:{}", self.width, self.height))
    }
}

impl Image {
    pub fn new(path: &str) -> Self {
        let image = ImageReader::open(path);

        if image.is_err() {
            panic!("Could not open image: {}", path);
        }

        let decoded_image = image.unwrap().decode().unwrap().to_rgb8();

        Self {
            width: decoded_image.width(),
            height: decoded_image.height(),
            data: decoded_image.to_vec(),
        }
    }

    pub fn parse_rgb_pixels(&self) -> Vec<RgbPixel> {
        let mut pixels: Vec<RgbPixel> = Vec::new();

        self.data.chunks(3).for_each(|chunk| {
            pixels.push(RgbPixel::new(chunk[0], chunk[1], chunk[2]));
        });

        pixels
    }
}

#[cfg(test)]
mod tests {
    use crate::image::Image;
    use test_case::test_case;

    #[test_case("data/demo.png", 300, 220)]
    fn decode_image(path: &str, width: u32, height: u32) {
        let image = Image::new(path);

        assert_eq!(image.width, width);
        assert_eq!(image.height, height);
    }
}
