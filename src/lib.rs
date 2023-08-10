#![feature(test)]

mod camera;
mod colour;
pub mod hittable;
pub mod image_encoder;
pub mod material;
mod point3;
mod random_world;
mod ray;
mod render;
mod vec3;

pub use camera::Camera;
pub use colour::Colour;
pub use point3::Point3;
pub use random_world::random_world;
pub use render::{render, RenderOptions};
pub use vec3::Vec3;

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use crate::{
        image_encoder::ImageEncoder, random_world, render, Camera, Point3, RenderOptions, Vec3,
    };

    struct TestImageEncoder;

    impl ImageEncoder for TestImageEncoder {
        fn new(_: u32, _: u32) -> Self {
            Self
        }

        fn save_image(&self, _: &str) -> Result<(), ()> {
            Ok(())
        }

        fn set_pixel(&mut self, _: u32, _: u32, _: crate::Colour) {
            ()
        }
    }

    fn create_test_render_options(aspect_ratio: f64) -> RenderOptions {
        let image_width = 60;
        RenderOptions {
            image_width: image_width,
            image_height: (image_width as f64 / aspect_ratio) as u32,
            samples: 32,
            max_bounces: 8,
            show_progress_bar: false,
            use_bvh: true,
        }
    }

    fn create_test_camera(aspect_ratio: f64) -> Camera {
        Camera::new(
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.1,
            10.0,
        )
    }

    #[bench]
    #[ignore]
    fn bench_random_world_no_bvh(b: &mut Bencher) {
        const ASPECT_RATIO: f64 = 3.0 / 2.0;

        let mut render_options = create_test_render_options(ASPECT_RATIO);
        render_options.use_bvh = false;

        let camera = create_test_camera(ASPECT_RATIO);
        let world = random_world();

        b.iter(move || {
            test::black_box(
                render::<TestImageEncoder>(render_options.clone(), camera.clone(), world.clone())
                    .unwrap(),
            );
        })
    }

    #[bench]
    fn bench_random_world(b: &mut Bencher) {
        const ASPECT_RATIO: f64 = 3.0 / 2.0;

        let render_options = create_test_render_options(ASPECT_RATIO);
        let camera = create_test_camera(ASPECT_RATIO);
        let world = random_world();

        b.iter(move || {
            test::black_box(
                render::<TestImageEncoder>(render_options.clone(), camera.clone(), world.clone())
                    .unwrap(),
            );
        })
    }
}
