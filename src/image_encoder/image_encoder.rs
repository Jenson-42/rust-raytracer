use std::path::Path;

use crate::Colour;

/// A wrapper trait around any image encoder you want to use.
pub trait ImageEncoder {
    fn new(width: u32, height: u32) -> Self;
    fn set_pixel(&mut self, x: u32, y: u32, colour: Colour);
    fn save_image<P>(&self, filename: P) -> Result<(), ()>
    where
        P: AsRef<Path>;
}
