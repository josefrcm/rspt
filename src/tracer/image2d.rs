use super::*;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub type Image2D = ndarray::Array2<Color>;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub fn new(width: usize, height: usize) -> Image2D {
    ndarray::Array2::zeros((height, width))
}

pub fn accum(lhs: &mut Image2D, rhs: &Image2D) {
    assert!(lhs.shape() == rhs.shape());

    lhs.zip_mut_with(rhs, |a, &b| *a += b);
}

pub fn scale(image: &mut Image2D, factor: usize) -> () {
    let s = 1.0 / (factor as f32);
    image.map_inplace(|a| *a = *a * s);
}

pub fn save_png(image: &Image2D, filename: &std::path::Path) -> image::ImageResult<()> {
    // Compute the scale factor
    let mut high = f32::NEG_INFINITY;
    //let high = self.pixels.max();
    for value in image.iter() {
        high = f32::max(high, value.x);
        high = f32::max(high, value.y);
        high = f32::max(high, value.z);
    }
    let scale = 1.0 / high;
    let gamma = 1.0 / 2.2;

    // Rescale the image to the range [0, 255]
    let width = image.ncols();
    let height = image.nrows();
    let mut buf = vec![0; 3 * width * height];
    for ((y, x), pixel) in image.indexed_iter() {
        let offset = 3 * (width * y + x);
        buf[offset + 0] = (255.0 * (scale * pixel.x).powf(gamma)) as u8;
        buf[offset + 1] = (255.0 * (scale * pixel.y).powf(gamma)) as u8;
        buf[offset + 2] = (255.0 * (scale * pixel.z).powf(gamma)) as u8;
    }

    // Save the image as PNG
    image::save_buffer(
        filename,
        &buf,
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
}

pub fn save_hdr(image: &Image2D, filename: &std::path::Path) -> image::ImageResult<()> {
    type RgbF32 = image::Rgb<f32>;

    // Rescale the image to the range [0, 255]
    let width = image.ncols();
    let height = image.nrows();
    let buf: Vec<RgbF32> = image
        .iter()
        .map(|pixel| RgbF32::from([pixel.x, pixel.y, pixel.z]))
        .collect();

    // Save the image as PNG
    let writer = std::fs::File::create(filename)?;
    let encoder = image::hdr::HDREncoder::new(writer);
    encoder.encode(&buf, width, height)
}
