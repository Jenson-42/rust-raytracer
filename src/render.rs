use std::error::Error;
use std::thread;

use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;

use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittable::{ArcHittable, Hittable};

use crate::image_encoder::ImageEncoder;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub image_width: u32,
    pub image_height: u32,
    pub samples: u32,
    pub max_bounces: u32,
    pub show_progress_bar: bool,
}

/// Get the colour of a ray sent out into the world.
fn ray_colour(ray: &Ray, world: &Vec<ArcHittable>, max_depth: u32) -> Colour {
    // Return black if we've reached the maximum number of bounces.
    if max_depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    // Check if the ray hits anything in the scene.
    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        // If it does, check if the material scatters the ray or absorbs it.
        if let Some(mat_record) = hit_record.material.scatter(ray, &hit_record) {
            // Mix the color of the original ray with the color of the scattered ray.
            return mat_record.attenuation
                * ray_colour(&mat_record.scattered, world, max_depth - 1);
        } else {
            // Return black if the ray was absorbed.
            return Colour::new(0.0, 0.0, 0.0);
        }
    }

    // Calculate the world colour if the ray doesn't hit anything.
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (Colour::new(0.9, 0.9, 0.9) * (1.0 - t)) + (Colour::new(0.5, 0.7, 1.0) * t)
}

/// Render a [Hittable] to a given image encoder using a camera.
pub fn render<O: ImageEncoder>(
    options: RenderOptions,
    camera: Camera,
    world: Vec<ArcHittable>,
) -> Result<O, Box<dyn Error>> {
    let RenderOptions {
        image_width,
        image_height,
        samples,
        max_bounces,
        show_progress_bar,
    } = options;

    let mut image_buffer = O::new(image_width, image_height);

    let bar = if show_progress_bar {
        let b = ProgressBar::new(image_height as u64 * 10);
        b.set_style(
            ProgressStyle::with_template("{elapsed} {wide_bar} {percent}% eta: {eta}")
                .expect("Progress bar style should be valid."),
        );
        Some(b)
    } else {
        None
    };

    // Store all of the handles to the individual threads.
    let mut handles = Vec::with_capacity(image_height as usize);

    // For each row in the image, create a thread to calculate it's pixels.
    for j in 0..image_height {
        let world = world.clone();
        let camera = camera.clone();
        let bar = bar.clone();

        let handle = thread::spawn(move || {
            let mut row = Vec::with_capacity(image_width as usize);

            for i in 0..image_width {
                let mut colour = Colour::new(0.0, 0.0, 0.0);
                let mut rng = rand::thread_rng();

                for _ in 0..samples {
                    let u = (f64::from(i) + rng.gen_range(0.0..1.0)) / f64::from(image_width - 1);
                    let v = (f64::from(j) + rng.gen_range(0.0..1.0)) / f64::from(image_height - 1);

                    let r = camera.get_ray(u, v);

                    colour += ray_colour(&r, &world, max_bounces) * (1.0 / f64::from(samples))
                }

                if let Some(ref bar) = bar {
                    if i % (image_width / 10) == 0 {
                        bar.inc(1);
                    }
                }

                row.push(colour);
            }

            (j, row)
        });

        handles.push(handle);
    }

    // Wait for each thread to finish and encode it's colour.
    // This is done serially so that ImageEncoder doesn't have to be thread-safe.
    for handle in handles {
        let (j, row) = handle.join().unwrap();
        for (i, colour) in row.iter().enumerate() {
            image_buffer.set_pixel(i as u32, image_height - (j as u32 + 1), *colour)
        }
    }

    Ok(image_buffer)
}
