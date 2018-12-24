use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::f32;
//use std::io::BufWriter;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<super::Color>
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

impl Image {
    pub fn new(width : usize, height : usize) -> Self {
        Image {
            width: width,
            height: height,
            pixels: vec![super::BLACK; width*height]
        }
    }



    pub fn accum(&mut self, other: &Vec<super::Color>) {
        assert!((self.width*self.height) == other.len());
        for i in 0..(self.width*self.height) {
            self.pixels[i].r += other[i].r;
            self.pixels[i].g += other[i].g;
            self.pixels[i].b += other[i].b;
        }
    }

    
    /*pub fn _set_pixel(&mut self, x : usize, y : usize, color : Color) {
        debug_assert!(x < self.width, "Pixel coordinates out of bounds!");
        debug_assert!(y < self.height, "Pixel coordinates out of bounds!");
        self.pixels[y*self.width + x] = color;
    }
    
    pub fn _get_pixel(&self, x : usize, y : usize) -> Color {
        debug_assert!(x < self.width, "Pixel coordinates out of bounds!");
        debug_assert!(y < self.height, "Pixel coordinates out of bounds!");
        self.pixels[y*self.width + x]
    }*/


    pub fn normalize(&mut self) -> () {
        let mut high = f32::NEG_INFINITY;
        
        for i in 0..(self.width*self.height) {
            high = f32::max(high, self.pixels[i].r);
            high = f32::max(high, self.pixels[i].g);
            high = f32::max(high, self.pixels[i].b);
        }
        
        for i in 0..(self.width*self.height) {
            self.pixels[i].r = self.pixels[i].r / high;
            self.pixels[i].g = self.pixels[i].g / high;
            self.pixels[i].b = self.pixels[i].b / high;
        }
    }


    pub fn save_tga(&self, filename: &std::path::Path) -> io::Result<()> {
        // Create the image header
        let mut header : [u8; 18] = [0; 18];
        header[0] = 0;                            // Number of characters in identification field
        header[1] = 0;                            // No color map
        header[2] = 2;                            // Uncompressed RGB
        header[3] = 0;                            // Color map specification - ignored
        header[4] = 0;
        header[5] = 0;
        header[6] = 0;
        header[7] = 0;
        header[8] = 0;                                    // X origin
        header[9] = 0;
        header[10] = 0;                                   // Y origin
        header[11] = 0;
        header[12] = (self.width & 0xff) as u8;           // Image width (little-endian 16 bit unsigned)
        header[13] = ((self.width & 0xff00) >> 8) as u8;
        header[14] = (self.height & 0xff) as u8;          // Image width (little-endian 16 bit unsigned)
        header[15] = ((self.height & 0xff00) >> 8) as u8;
        header[16] = 24;                                  // Bits per pixel
        header[17] = 0;                                   // Coordinate origin on the upper-left corner

        // Create the image body
        // Note: TGA files are written upside down
        let mut body : Vec<u8> = Vec::with_capacity(self.width*self.height*3);
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let i = y*self.width + x;
                body.push((255.0*self.pixels[i].b).round() as u8);
                body.push((255.0*self.pixels[i].g).round() as u8);
                body.push((255.0*self.pixels[i].r).round() as u8);
            }
        }

        // Write the image
        let mut buffer = File::create(filename)?;
        buffer.write(&header)?;
        buffer.write(&body)?;
        Ok(())
    }
}
