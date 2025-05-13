use crate::rtweekend::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::{self, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,

    u: Vec3,
    v: Vec3,
    w: Vec3,
    pixel_sample_scale: f64,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let mut cam = Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            u: Point3::default(),
            v: Point3::default(),
            w: Point3::default(),
            pixel_sample_scale: 1.0 / samples_per_pixel as f64,
            image_height: 0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        };
        cam.initialize();
        cam
    }

    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio)
            .round()
            .max(1.0) as u32;
        //self.lookfrom = Point3::zero();
        //self.lookat = Point3::new(0.0, 0.0, -1.0);
        //self.vup = Vec3::new(0.0, 1.0, 0.0);
        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        self.w = (self.lookfrom - self.lookat).normalized();
        self.u = self.vup.cross(self.w).normalized();
        self.v = self.w.cross(self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left = self.center
            - self.focus_dist * self.w
            - viewport_u / 2.0                     // Move to left edge
            - viewport_v / 2.0; // Move to top edge (since viewport_v is downwards)

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
    /*
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
    */
    pub fn render<W: Hittable + Sync>(&self, world: &W) -> io::Result<()> {
        // `world` needs to be Sync because it's accessed by multiple threads.
        // `self` is also accessed by multiple threads (for its methods and fields),
        // so Camera itself needs to be Sync (which it should be if its fields are).

        let num_pixels = (self.image_width * self.image_height) as usize;

        // --- Start of logging ---
        eprintln!("Starting parallel render...");
        eprintln!(
            "Image Dimensions: {}x{}",
            self.image_width, self.image_height
        );
        eprintln!("Samples per pixel: {}", self.samples_per_pixel);
        eprintln!("Max depth: {}", self.max_depth);
        // --- End of logging ---

        // Calculate all pixel colors in parallel
        let pixel_colors: Vec<Color> = (0..num_pixels)
            .into_par_iter() // Convert range to parallel iterator
            .map(|pixel_idx| {
                // Calculate (i, j) from the flat pixel_idx
                // These are the logical pixel coordinates (0 to width-1, 0 to height-1)
                let i = (pixel_idx % self.image_width as usize) as u32;
                // For PPM, j=0 is the top row.
                // If pixel_idx=0 is top-left, then j = (pixel_idx / self.image_width as usize) as u32;
                // maps correctly.
                let j_for_ray = (pixel_idx / self.image_width as usize) as u32;

                let mut accumulated_color = Color::zero();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j_for_ray); // Use the logical j for ray generation
                    accumulated_color += self.ray_color(&r, self.max_depth, world);
                }
                accumulated_color * self.pixel_sample_scale
            })
            .collect(); // Collect results into a Vec

        eprintln!("\nParallel computation finished. Writing to output...");

        // Write to stdout (or a file) sequentially
        let mut output_buffer = BufWriter::new(io::stdout().lock()); // Or File::create for file output
        // If writing to a file, e.g., "image.ppm":
        // let mut output_buffer = BufWriter::new(File::create("image.ppm")?);

        writeln!(output_buffer, "P3")?;
        writeln!(output_buffer, "{} {}", self.image_width, self.image_height)?;
        writeln!(output_buffer, "255")?;

        // Iterate through the collected pixel_colors and write them out.
        // PPM writes rows from top to bottom.
        // Our pixel_colors Vec is ordered such that pixel_colors[0] is pixel (0,0) [top-left],
        // pixel_colors[1] is (1,0), ..., pixel_colors[width-1] is (width-1,0),
        // pixel_colors[width] is (0,1), etc.
        for pixel_color in pixel_colors {
            write_color(&mut output_buffer, pixel_color)?;
        }

        output_buffer.flush()?; // Ensure all data is written
        eprintln!("\nDone. Output complete.");

        Ok(())
    }

    fn ray_color<W: Hittable>(&self, r: &Ray, depth: u32, world: &W) -> Color {
        // Define the interval for valid hits. Use a small t_min to avoid self-intersection.
        if depth <= 0 {
            return Color::zero();
        }

        let hit_interval = Interval::new(0.001, INFINITY);

        if let Some(rec) = world.hit(r, hit_interval) {
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
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        if self.samples_per_pixel == 1 {
            return Vec3::zero();
        }
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
