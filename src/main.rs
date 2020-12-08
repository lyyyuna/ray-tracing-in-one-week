#![allow(dead_code)]

use std::iter::from_fn;

mod ppm;

fn main() {
    println!("Hello, world!");

    let mut col = 0usize;
    let mut row = 256usize;

    let mut picture: ppm::Image = from_fn(|| {
        if col == 256 {
            col = 0;
            row -= 1;
        }

        if row == 0 {
            return None;
        }

        let c = ppm::Color::newf(col as f32 / 255.0, (row - 1) as f32 / 255.0, 0.25);
        col += 1;
        Some(c)
    }).collect();

    picture.reshape(256).unwrap();
    picture.save("first.ppm").unwrap();
}
