use rand::{rngs::ThreadRng, Rng};

use crate::vec3::Vec3;

pub type Colour = Vec3<f64>;

impl Into<[u8; 3]> for Colour {
    fn into(self) -> [u8; 3] {
        let multiplier = 255.999;
        let r = (multiplier * self.0.sqrt()) as u8;
        let g = (multiplier * self.1.sqrt()) as u8;
        let b = (multiplier * self.2.sqrt()) as u8;

        [r, g, b]
    }
}

impl Colour {
    pub fn random(rng: &mut ThreadRng) -> Self {
        let r = rng.gen_range(0.0..1.0);
        let g = rng.gen_range(0.0..1.0);
        let b = rng.gen_range(0.0..1.0);

        Self(r, g, b)
    }
}
