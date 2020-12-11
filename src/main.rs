#![allow(dead_code)]
use env_logger;

mod color;
mod image;
mod vec3;
mod ray;

fn main() {
    env_logger::init();

    let painter = image::Painter::new(256, 256, "first.ppm").unwrap();
    painter.draw(|row, col| color::Color::new(col as u8, 255 - row as u8, 64)).unwrap();
}
