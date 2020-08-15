use std::f32;
use std::io;

use super::*;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width: width,
            height: height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn accum(&mut self, other: &Vec<Color>) {
        assert!((self.width * self.height) == other.len());
        for i in 0..(self.width * self.height) {
            self.pixels[i].r += other[i].r;
            self.pixels[i].g += other[i].g;
            self.pixels[i].b += other[i].b;
        }
    }

    pub fn scale(&mut self, factor: usize) -> () {
        let s = 1.0 / (factor as f32);
        for i in 0..(self.width * self.height) {
            self.pixels[i] = s * self.pixels[i];
        }
    }

    pub fn save_png(&self, filename: &std::path::Path) -> image::ImageResult<()> {
        // Compute the scale factor
        let mut high = f32::NEG_INFINITY;
        for i in 0..(self.width * self.height) {
            high = f32::max(high, self.pixels[i].r);
            high = f32::max(high, self.pixels[i].g);
            high = f32::max(high, self.pixels[i].b);
        }
        let scale = 1.0 / high;
        let gamma = 1.0 / 2.2;

        // Reescale the image to the range [0, 255]
        let mut buf = vec![0; 3 * self.width * self.height];
        for i in 0..(self.width * self.height) {
            buf[3 * i + 0] = (255.0 * (scale * self.pixels[i].r).powf(gamma)) as u8;
            buf[3 * i + 1] = (255.0 * (scale * self.pixels[i].g).powf(gamma)) as u8;
            buf[3 * i + 2] = (255.0 * (scale * self.pixels[i].b).powf(gamma)) as u8;
        }

        // Save the image as PNG
        image::save_buffer(
            filename,
            &buf,
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgb8,
        )
    }
}
