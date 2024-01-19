mod vector3;
mod colour;
mod ray;

use std::fs::File;
use std::io::{ Write, self };
use std::ops::Mul;

use colour::{ Colour, write_colour };

use vector3::{ Vector3, Point3, unit_vector };

use crate::ray::Ray;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = ((image_width as f64) / aspect_ratio) as i32;
    image_height = image_height.max(1);

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width / image_height) as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut image_file = File::create("image.ppm").expect("file creation failed");

    println!("P3\nImage width: {}\nImage height: {}\n255\n", image_width, image_height);

    image_file
        .write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
        .expect("write failed");

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + pixel_delta_u.mul(i as f64) + pixel_delta_v.mul(j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_colour = ray_color(&r);

            write_colour(&mut image_file, pixel_colour);
        }
    }

    println!("\rDone.                 \n");
}

fn ray_color(r: &Ray) -> Colour {
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0);
}
