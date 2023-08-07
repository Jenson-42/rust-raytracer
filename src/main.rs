use raytracing_in_a_weekend::{
    image_encoder::{ImageCrateRGBu8Encoder, ImageEncoder},
    random_world, render, Camera, Point3, RenderOptions, Vec3,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Image output options:
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 2400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const FILENAME: &str = "output.png";
    // The image encoder to use. This can be any type that implements ImageEncoder.
    type ImageEncoder = ImageCrateRGBu8Encoder;

    // Camera options:
    const VERTICAL_FIELD_OF_VIEW: f64 = 20.0;
    const FOCUS_DISTANCE: f64 = 10.0;
    const APERTURE: f64 = 0.1;

    // Renderer options:
    const SAMPLES: u32 = 256;
    const MAX_BOUNCES: u32 = 16;

    // Generate a scene to render.
    let world = random_world();

    let render_options: RenderOptions = RenderOptions {
        image_width: IMAGE_WIDTH,
        image_height: IMAGE_HEIGHT,
        samples: SAMPLES,
        max_bounces: MAX_BOUNCES,
        show_progress_bar: true,
    };

    // Generate a camera at a cool angle.
    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        VERTICAL_FIELD_OF_VIEW,
        ASPECT_RATIO,
        APERTURE,
        FOCUS_DISTANCE,
    );

    render::<ImageEncoder>(render_options, camera, world)?
        .save_image(FILENAME)
        .unwrap();

    Ok(())
}
