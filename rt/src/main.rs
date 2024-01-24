mod vector3;
mod colour;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;
mod interval;
mod camera;
mod material;

use std::f64::consts::PI;
use std::rc::Rc;

use camera::Camera;

use colour::Colour;
use material::{ Lambertian, Metal };
use vector3::{ Point3, Vector3 };

use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2)));

    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // let r = (PI / 4.0).cos();

    // let material_left = Rc::new(Lambertian::new(Colour::new(0.0, 0.0, 1.0)));
    // let material_right = Rc::new(Lambertian::new(Colour::new(1.0, 0.0, 0.0)));

    // world.add(Rc::new(Sphere::new(Point3::new(-r, 0.0, -1.0), r, material_left)));
    // world.add(Rc::new(Sphere::new(Point3::new(r, 0.0, -1.0), r, material_right)));

    // Camera
    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    cam.render(&world)
}
