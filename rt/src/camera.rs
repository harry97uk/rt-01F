use std::{ f64::INFINITY, fs::File, io::{ self, Write } };

use crate::{
    hittable::{ Hittable, HitRecord },
    ray::Ray,
    colour::{ Colour, write_colour },
    interval::Interval,
    vector3::{ unit_vector, Point3, Vector3, cross },
    rtweekend::{ random_f64, degrees_to_radians },
};

pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: i32, // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32, // Maximum number of ray bounces into scene
    pub vfov: f64, // Vertical view angle (field of view)
    pub lookfrom: Point3, // Point camera is looking from
    pub lookat: Point3, // Point camera is looking at
    pub vup: Vector3, // Camera-relative "up" direction
    pub brightness: f64,

    image_height: i32, // Rendered image height
    centre: Point3, // camera centre
    pixel00_loc: Point3, // Location of pixel 0,0
    pixel_delta_u: Vector3, // Offset to pixel to the right
    pixel_delta_v: Vector3, // Offset to pixel below
    u: Vector3,
    v: Vector3,
    w: Vector3, // Camera frame basis vectors
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 0.0,
            image_width: 0,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::default(),
            vup: Vector3::new(0.0, 1.0, 0.0),
            image_height: 0,
            centre: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vector3::default(),
            pixel_delta_v: Vector3::default(),
            u: Vector3::default(),
            v: Vector3::default(),
            w: Vector3::default(),
            brightness: 1.0,
        }
    }
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialise();

        let mut image_file = File::create("image.ppm").expect("file creation failed");

        println!(
            "P3\nImage width: {}\nImage height: {}\n255\n",
            self.image_width,
            self.image_height
        );

        image_file
            .write(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())
            .expect("write failed");

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - j);
            io::stdout().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_colour += self.ray_colour(&r, self.max_depth, world);
                }
                write_colour(&mut image_file, pixel_colour, self.samples_per_pixel);
            }
        }

        println!("\rDone.                 \n");
    }

    fn initialise(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = ((self.image_width as f64) / self.aspect_ratio).round() as i32;
        self.image_height = self.image_height.max(1);

        self.centre = self.lookfrom;

        // Determine viewport dimensions.
        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.centre - focal_length * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.

        let pixel_center =
            self.pixel00_loc + (i as f64) * self.pixel_delta_u + (j as f64) * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.centre;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn pixel_sample_square(&self) -> Vector3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + random_f64();
        let py = -0.5 + random_f64();
        return px * self.pixel_delta_u + py * self.pixel_delta_v;
    }

    fn ray_colour(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Colour {
        let mut rec = HitRecord::default();

        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Colour::default();
        }

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered: Ray = Ray::new(Vector3::default(), Vector3::default());
            let mut attenuation: Colour = Colour::default();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_colour(&scattered, depth - 1, world);
            }
            return Colour::default();
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (self.brightness - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0);
    }
}
