use std::{ f64::INFINITY, fs::File, io::{ self, Write }, ops::Mul };

use crate::{
    hittable::{ Hittable, HitRecord },
    ray::Ray,
    colour::{ Colour, write_colour },
    interval::Interval,
    vector3::{ unit_vector, Point3, Vector3 },
};

pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: i32, // Rendered image width in pixel count
    image_height: i32, // Rendered image height
    centre: Point3, // camera centre
    pixel00_loc: Point3, // Location of pixel 0,0
    pixel_delta_u: Vector3, // Offset to pixel to the right
    pixel_delta_v: Vector3, // Offset to pixel below
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 0.0,
            image_width: 0,
            image_height: 0,
            centre: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vector3::default(),
            pixel_delta_v: Vector3::default(),
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
                let pixel_center =
                    self.pixel00_loc +
                    self.pixel_delta_u.mul(i as f64) +
                    self.pixel_delta_v.mul(j as f64);
                let ray_direction = pixel_center - self.centre;
                let r = Ray::new(self.centre, ray_direction);

                let pixel_colour = self.ray_colour(&r, world);

                write_colour(&mut image_file, pixel_colour);
            }
        }

        println!("\rDone.                 \n");
    }

    fn initialise(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = ((self.image_width as f64) / self.aspect_ratio).round() as i32;
        self.image_height = self.image_height.max(1);

        self.centre = Point3::default();

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.centre -
            Vector3::new(0.0, 0.0, focal_length) -
            viewport_u / 2.0 -
            viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_colour(&self, r: &Ray, world: &dyn Hittable) -> Colour {
        let mut rec = HitRecord::default();

        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0);
    }
}
