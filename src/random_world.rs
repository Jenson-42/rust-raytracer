use crate::{
    hittable::{
        hittables::{Disk, Sphere},
        ArcHittable,
    },
    material::{
        materials::{Checkerboard, Dielectric, Lambertian, Metal},
        ArcMaterial,
    },
    Colour, Point3, Vec3,
};
use rand::{rngs::ThreadRng, Rng};

fn random_f64(rng: &mut ThreadRng) -> f64 {
    rng.gen_range(0.0..1.0)
}

/// Generate a scene of random spheres to be rendered.
/// This is included as part of the library because it is used for benchmarking.
pub fn random_world() -> Vec<ArcHittable> {
    // The materials to be used by the non-random spheres in the scene.
    // They're only in a Vec because I'm too lazy to specify the type for each one.
    let materials: Vec<ArcMaterial> = vec![
        Checkerboard::new(
            0.5,
            Lambertian::new(Colour::new(0.7, 0.7, 0.7)).into(),
            Metal::new(Colour::new(0.1, 0.1, 0.1), 0.5).into(),
        )
        .into(),
        Lambertian::new(Colour::new(0.4, 0.2, 0.1)).into(),
        Dielectric::new(1.5).into(),
        Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0).into(),
    ];

    // Locations of the spheres so they can be avoided by the smaller ones.
    let ball_locations = vec![
        Point3::new(-4.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(4.0, 1.0, 0.0),
    ];

    // Create a scene with a ground as well as a sphere for each material.
    let mut world: Vec<ArcHittable> = vec![
        // Ground disk.
        Disk::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            15.0,
            &materials[0],
        )
        .into(),
        // Matte Ball.
        Sphere::new(ball_locations[0], 1.0, &materials[1]).into(),
        // Glass ball.
        Sphere::new(ball_locations[1], 1.0, &materials[2]).into(),
        // Metal ball.
        Sphere::new(ball_locations[2], 1.0, &materials[3]).into(),
    ];

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let radius = 0.1 + (random_f64(&mut rng) / 8.0);

            let center = Point3::new(
                a as f64 + 0.9 * random_f64(&mut rng),
                radius,
                b as f64 + 0.9 * random_f64(&mut rng),
            );

            // Discard any spheres not on the disk.
            if center.length() > 15.0 {
                continue;
            }

            // Discard any spheres touching the big ones.
            if ball_locations.iter().any(|p| (center - *p).length() < 1.1) {
                continue;
            }

            // Choose a random material for the sphere.
            let material: ArcMaterial = match random_f64(&mut rng) {
                mat if mat < 0.9 => {
                    let albedo = Colour::random(&mut rng);
                    Lambertian::new(albedo).into()
                }
                mat if mat < 0.95 => {
                    let albedo = Colour::random(&mut rng);
                    Metal::new(albedo, 0.2).into()
                }
                _ => Dielectric::new(1.5).into(),
            };

            // Push the new sphere into the world.
            world.push(Sphere::new(center, radius, &material).into());
        }
    }

    world
}
