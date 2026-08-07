use crate::image::Image;

mod hsl_pixel;
mod image;
mod rgb_pixel;
mod sound;

fn main() {
    let image = Image::new("data/demo.png");

    for pixel in image.parse_rgb_pixels() {
        let hsl = pixel.to_hsl_pixel();
        let sound = hsl.to_sound();
        
        println!("{}", sound);
    }
}
