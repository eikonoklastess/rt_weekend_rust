use crate::rtweekend::*;
use std::io::{self, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    pixel_sample_scale: f64,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let mut cam = Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            pixel_sample_scale: 1.0 / samples_per_pixel as f64,
            image_height: 0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        };
        cam.initialize();
        cam
    }

    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio)
            .round()
            .max(1.0) as u32;
        self.center = Point3::zero();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left = self.center
            - Vec3::new(0.0, 0.0, focal_length) // Vector to the focal plane
            - viewport_u / 2.0                     // Move to left edge
            - viewport_v / 2.0; // Move to top edge (since viewport_v is downwards)

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render<W: Hittable>(&self, world: &W) -> io::Result<()> {
        let mut stdout_buffer = io::BufWriter::new(io::stdout().lock());
        writeln!(stdout_buffer, "P3")?;
        writeln!(stdout_buffer, "{} {}", self.image_width, self.image_height)?;
        writeln!(stdout_buffer, "255")?;

        let mut stderr = io::stderr();

        for j in 0..self.image_height {
            // Progress indicator
            if j % 20 == 0 || j == self.image_height - 1 {
                // Update less frequently
                eprint!("\rScanlines remaining: {:<4}", self.image_height - 1 - j);
                stderr.flush()?;
            }

            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }

                write_color(&mut stdout_buffer, pixel_color * self.pixel_sample_scale)?;
            }
        }

        eprintln!("\rDone.                          ");
        stderr.flush()?;
        // stdout_buffer flushes on drop

        Ok(())
    }

    fn ray_color<W: Hittable>(&self, r: &Ray, depth: u32, world: &W) -> Color {
        // Define the interval for valid hits. Use a small t_min to avoid self-intersection.
        if depth <= 0 {
            return Color::zero();
        }

        let hit_interval = Interval::new(0.001, INFINITY);

        if let Some(rec) = world.hit(r, Interval::new(hit_interval.min, hit_interval.max)) {
            if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }
            return Color::zero();
        }

        // If no hit, it's the background (sky gradient)
        let unit_direction = r.dir.normalized();
        let a = 0.5 * (unit_direction.y + 1.0); // Using public field .y
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        if self.samples_per_pixel == 1 {
            return Vec3::zero();
        }
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }
}
