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
            Lambertian::new(Colour::new(0.5, 0.5, 0.5)).into(),
            Lambertian::new(Colour::new(0.9, 0.9, 0.9)).into(),
        )
        .into(),
        // Lambertian::new_arc(Colour::new(0.5, 0.5, 0.5)),
        Lambertian::new(Colour::new(0.4, 0.2, 0.1)).into(),
        Dielectric::new(1.5).into(),
        Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0).into(),
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
        Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, &materials[1]).into(),
        // Glass ball.
        Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, &materials[2]).into(),
        // Metal ball.
        Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, &materials[3]).into(),
    ];

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(&mut rng),
                0.2,
                b as f64 + 0.9 * random_f64(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() < 0.9 {
                continue;
            }

            let material: ArcMaterial = match random_f64(&mut rng) {
                mat if mat < 0.8 => {
                    let albedo = Colour::random(&mut rng);
                    Lambertian::new(albedo).into()
                }
                mat if mat < 0.95 => {
                    let albedo = Colour::random(&mut rng);
                    Lambertian::new(albedo).into()
                }
                _ => Dielectric::new(1.5).into(),
            };

            world.push(Sphere::new(center, 0.2, &material).into());
        }
    }

    world
}
