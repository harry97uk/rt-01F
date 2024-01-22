mod vector3;
mod colour;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;
mod interval;
mod camera;

use std::f64::INFINITY;
use std::fs::File;
use std::io::{ Write, self };
use std::ops::Mul;
use std::rc::Rc;

use camera::Camera;
use colour::{ Colour, write_colour };

use hittable::{ HitRecord, Hittable };
use interval::Interval;
use vector3::{ Vector3, Point3, unit_vector };

use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;

fn main() {
    // World
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world)
}
