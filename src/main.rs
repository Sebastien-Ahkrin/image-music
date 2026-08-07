use std::thread::sleep;
use std::time::Duration;
use crate::image::Image;
use crate::player::Player;

mod hsl_pixel;
mod image;
mod player;
mod rgb_pixel;
mod sound;

fn main() {
    let image = Image::new("data/demo.png");
    let player = Player::default();

    for pixel in image.parse_rgb_pixels() {
        let pixel_sound = pixel.to_hsl_pixel().to_sound();
        player.play_note(&pixel_sound, 50);

        sleep(Duration::from_millis(50));

        println!("{pixel_sound}")
    }
}
