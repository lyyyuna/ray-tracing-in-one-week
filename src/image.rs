use std::{fs::File, io::BufWriter, path::Path, vec};
use std::io::Write;
use log::info;

use super::color::Color;

pub struct Image {
    width: usize,
    height: usize,
    colors: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let colors = vec![Color::default(); width * height];
        Self {
            width, height, colors
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        write!(&mut file, "P3\n{width} {height}\n255\n", width = self.width, height = self.height)?;

        for row in 0..self.height {
            for col in 0..self.width {
                let index = row * self.width + col;
                let color = self.colors[index].i();
                write!(&mut file, "{r} {g} {b}\n", r = color.r, g = color.g, b = color.b)?;
            }
        }

        Ok(())
    }

    pub fn reshape(&mut self, width: usize) -> Result<(), ()> {
        if self.colors.len() % width == 0 {
            self.width = width;
            self.height = self.colors.len() / width;
            Ok(())
        } else {
            Err(())
        }
    }
}

pub struct Painter {
    width: usize,
    height: usize,
    // file: File,
}

impl Painter {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn draw<P, F>(&self, path: P, mut f: F) -> std::io::Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(usize, usize) -> Color, {
        let mut file = BufWriter::new(File::create(path.as_ref())?);
        write!(&mut file, "P3\n{width} {height}\n255\n", width = self.width, height = self.height)?;

        for row in 0..self.height {
            info!("scan line remaining: {}", self.height - row);
            for col in 0..self.width {
                let color = f(row, col);
                let color = color.i();
                write!(&mut file, "{r} {g} {b}\n", r = color.r, g = color.g, b = color.b)?;
                // 16 KB
                if file.buffer().len() >= 16 << 10 {
                    file.flush()?;
                }
            }
        }

        Ok(())
    }
}