use crate::image::Image;

mod image;

fn main() {
    let image = Image::new("data/demo.png");
    println!("{image}");
}
