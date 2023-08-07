use crate::{colour::Colour, ray::Ray};

pub struct MaterialRecord {
    pub attenuation: Colour,
    pub scattered: Ray,
}
