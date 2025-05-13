mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use crate::rtweekend::*;
use std::io;
use std::sync::Arc; // Make sure PI is available

// Assuming your imports for Color, Point3, Vec3, Lambertian, Dielectric, Metal,
// Sphere, HittableList, Camera, etc., are at the top of your main.rs

fn main() -> io::Result<()> {
    // --- Materials ---
    // Ground
    let material_ground_reflective_dark = Arc::new(Metal::new(Color::new(0.1, 0.1, 0.15), 0.05)); // Dark, slightly fuzzy mirror
    // let material_ground_diffuse_dark = Arc::new(Lambertian::new(Color::new(0.05, 0.05, 0.05))); // Alternative very dark diffuse

    // Primary Orbs
    let material_large_glass = Arc::new(Dielectric::new(1.5)); // Standard glass
    let material_large_metal_gold = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0)); // Polished gold
    let material_large_metal_silver = Arc::new(Metal::new(Color::new(0.01, 0.0, 0.6), 0.0)); // Slightly fuzzy silver

    // Accent / Small Orbs
    let material_diffuse_blue = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.7)));
    let material_diffuse_red = Arc::new(Lambertian::new(Color::new(0.7, 0.1, 0.1)));
    let material_metal_copper_fuzzy = Arc::new(Metal::new(Color::new(0.7, 0.3, 0.1), 0.4));
    let material_small_glass_bubbles = Arc::new(Dielectric::new(1.3)); // Slightly different IOR for variety
    let material_glowing_emitter_placeholder = Arc::new(Lambertian::new(Color::new(0.9, 0.9, 0.7))); // Brighter diffuse to simulate glow

    // --- World ---
    let mut world = HittableList::new();

    // Ground Plane (Large Sphere)
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0), // Y very low to make it flat
        1000.0,
        material_ground_reflective_dark.clone(), // Use clone for Arc if used elsewhere, or just pass
    )));

    // --- Primary Large Spheres ---
    // Central Glass Orb
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_large_glass.clone(),
    )));

    // Left Gold Metal Orb
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_large_metal_gold.clone(),
    )));

    // Right Silver Metal Orb
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_large_metal_silver.clone(),
    )));

    // --- Scattered Smaller Spheres ---
    // This loop creates a field of smaller, randomly placed and materialized spheres.
    // Adjust the range (-10 to 10) and density as desired.

    let small_sphere_radius = 0.2;
    for a in -3..3 {
        for b in -3..3 {
            let choose_mat = rand::random::<f64>(); // Using rand crate for random numbers
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                small_sphere_radius, // Place them just above the ground (y=0)
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            // Ensure small spheres don't overlap too much with the large ones
            if (center - Point3::new(0.0, 1.0, 0.0)).length() > 1.0 + small_sphere_radius
                && (center - Point3::new(-4.0, 1.0, 0.0)).length() > 1.0 + small_sphere_radius
                && (center - Point3::new(4.0, 1.0, 0.0)).length() > 1.0 + small_sphere_radius
            {
                let sphere_material: Arc<dyn Material + Send + Sync>;
                if choose_mat < 0.3 {
                    // 30% diffuse
                    let albedo = Color::random() * Color::random(); // Random diffuse color
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.6 {
                    // 30% metal
                    let albedo = Color::new(
                        random_f64_range(0.5, 1.0),
                        random_f64_range(0.5, 1.0),
                        random_f64_range(0.5, 1.0),
                    );
                    let fuzz = random_f64_range(0.0, 0.5); // Using your utility if available, else rand::random
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else if choose_mat < 0.8 {
                    // 20% glass
                    sphere_material = material_small_glass_bubbles.clone();
                } else {
                    // 20% "glowing" (brighter diffuse)
                    sphere_material = material_glowing_emitter_placeholder.clone();
                }
                world.add(Arc::new(Sphere::new(
                    center,
                    small_sphere_radius,
                    sphere_material,
                )));
            }
        }
    }

    // --- Camera Settings ---
    // High quality settings - WILL BE SLOW!
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 600; // Higher resolution (e.g., 1920 or 2560)
    let sample_per_pixel: u32 = 10; // Significantly more samples for AA and soft effects
    // Consider 1000-5000 for "final" quality if you have patience
    let max_depth: u32 = 10; // Good depth for complex interactions

    // Camera positioning for a dramatic, slightly low angle shot
    let vfov = 25.0; // A bit wider than your example, but not too extreme
    let lookfrom = Point3::new(8.0, 2.5, 10.0); // Further back, slightly elevated, off to the side
    let lookat = Point3::new(0.0, 0.5, 0.0); // Look towards the center of the large orbs, slightly above ground
    let vup = Vec3::new(0.0, 1.0, 0.0);

    // Depth of field settings - focus on one of the main orbs or a point between them
    let defocus_angle = 0.8; // Subtle defocus, increase for more blur (e.g., 1.0 to 2.0)
    let focus_dist = (lookfrom - Point3::new(0.0, 1.0, 0.0)).length(); // Focus on the central large sphere

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        sample_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    // --- Render ---
    eprintln!("Starting render with high quality settings...");
    eprintln!(
        "Image Width: {}, Samples/Pixel: {}, Max Depth: {}",
        image_width, sample_per_pixel, max_depth
    );
    cam.render(&world)?;
    eprintln!("Render finished!");

    Ok(())
}

// Make sure you have random utilities. If not, you can use the `rand` crate:
// Add `rand = "0.8"` to your Cargo.toml
// And `use rand::Rng;` in your file.
// Then replace `rand::random::<f64>()` with `rand::thread_rng().gen::<f64>()`
// and `rand::random_range(min, max)` with `rand::thread_rng().gen_range(min..max)`
// Or adapt to your Vec3::random() and Vec3::random_range() if you have them.
