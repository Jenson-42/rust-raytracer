use magnetite::{
    image_encoder::{ImageCrateRGBu8Encoder, ImageEncoder},
    random_world, render, Camera, Point3, RenderOptions, Vec3,
};
use std::{error::Error, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct RandomBallsCli {
    /// Width of the output image. The aspect ratio is 3:2 so setting this to 600 results in a 600x400px image.
    #[arg(short, long, default_value_t = 600)]
    width: u16,

    /// Number of samples to use for each pixel.
    #[arg(short, long, default_value_t = 256)]
    samples: u16,

    /// Output file directory.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let RandomBallsCli {
        width,
        samples,
        output,
    } = RandomBallsCli::parse();

    // Image output options:
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    // const width: u32 = 1200;
    let image_height: u32 = (width as f64 / ASPECT_RATIO) as u32;
    let filename = output.unwrap_or(PathBuf::from("/output.png"));
    // The image encoder to use. This can be any type that implements ImageEncoder.
    type ImageEncoder = ImageCrateRGBu8Encoder;

    // Camera options:
    const VERTICAL_FIELD_OF_VIEW: f64 = 20.0;
    const FOCUS_DISTANCE: f64 = 10.0;
    const APERTURE: f64 = 0.1;

    // Renderer options:
    const MAX_BOUNCES: u32 = 16;

    // Generate a scene to render.
    let world = random_world();

    let render_options: RenderOptions = RenderOptions {
        image_width: width as u32,
        image_height,
        samples: samples as u32,
        max_bounces: MAX_BOUNCES,
        show_progress_bar: true,
        use_bvh: true,
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
        .save_image(filename)
        .unwrap();

    Ok(())
}
