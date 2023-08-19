use std::path::Path;

use image::{ImageBuffer, Pixel, Rgb};

use super::ImageEncoder;

/// A thin wrapper around the image crate imgbuf to use with the renderer.
/// This can be used with any pixel type you want.
pub struct ImageCrateEncoder<P: Pixel> {
    imgbuf: ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>,
}

impl ImageEncoder for ImageCrateEncoder<Rgb<u8>> {
    fn new(width: u32, height: u32) -> Self {
        Self {
            imgbuf: ImageBuffer::new(width, height),
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, colour: crate::Colour) {
        *self.imgbuf.get_pixel_mut(x, y) = Rgb(colour.into());
    }

    fn save_image<P>(&self, filename: P) -> Result<(), ()>
    where
        P: AsRef<Path>,
    {
        match self.imgbuf.save(filename) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

/// A type alias for an encoder using the [image] crate with [Rgb]<[u8]> pixels.
pub type ImageCrateRGBu8Encoder = ImageCrateEncoder<Rgb<u8>>;
