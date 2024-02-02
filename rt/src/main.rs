#![recursion_limit = "2048"]
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
mod quad;
mod aabb;
mod bvh;
mod cylinder;

use std::f64::consts::PI;
use std::rc::Rc;

use bvh::BvhNode;
use camera::Camera;

use colour::Colour;
use cylinder::Cylinder;
use material::{ Lambertian, Metal };
use quad::{ Plane, cuboid };
use vector3::{ Point3, Vector3 };

use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;

fn main() {
    // World
    let mut world = HittableList::new();

    // Materials
    // let left_red = Rc::new(Lambertian::new(Colour::new(1.0, 0.2, 0.2)));
    // let back_green = Rc::new(Lambertian::new(Colour::new(0.2, 1.0, 0.2)));
    // let right_blue = Rc::new(Lambertian::new(Colour::new(0.2, 0.2, 1.0)));
    // let upper_orange = Rc::new(Lambertian::new(Colour::new(1.0, 0.5, 0.0)));
    // let lower_teal = Rc::new(Lambertian::new(Colour::new(0.2, 0.8, 0.8)));

    // // Quads
    // world.add(
    //     Rc::new(
    //         Plane::new(
    //             Point3::new(-3.0, -2.0, 5.0),
    //             Vector3::new(0.0, 0.0, -4.0),
    //             Vector3::new(0.0, 4.0, 0.0),
    //             left_red
    //         )
    //     )
    // );
    // world.add(
    //     Rc::new(
    //         Plane::new(
    //             Point3::new(-2.0, -2.0, 0.0),
    //             Vector3::new(4.0, 0.0, 0.0),
    //             Vector3::new(0.0, 4.0, 0.0),
    //             back_green
    //         )
    //     )
    // );
    // world.add(
    //     Rc::new(
    //         Plane::new(
    //             Point3::new(3.0, -2.0, 1.0),
    //             Vector3::new(0.0, 0.0, 4.0),
    //             Vector3::new(0.0, 4.0, 0.0),
    //             right_blue
    //         )
    //     )
    // );
    // world.add(
    //     Rc::new(
    //         Plane::new(
    //             Point3::new(-2.0, 3.0, 1.0),
    //             Vector3::new(4.0, 0.0, 0.0),
    //             Vector3::new(0.0, 0.0, 4.0),
    //             upper_orange
    //         )
    //     )
    // );
    // world.add(
    //     Rc::new(
    //         Plane::new(
    //             Point3::new(-2.0, -3.0, 5.0),
    //             Vector3::new(4.0, 0.0, 0.0),
    //             Vector3::new(0.0, 0.0, -4.0),
    //             lower_teal
    //         )
    //     )
    // );

    let material_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2)));

    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    //world.add(cuboid(Point3::new(0.0, 0.0, -1.0), Point3::new(0.5, 0.5, -0.5), material_center));
    world.add(Rc::new(Sphere::new(Point3::new(-2.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(2.0, 0.0, -1.0), 0.5, material_right)));

    // Create a cylinder
    let cylinder_center = Point3::new(0.0, 1.0, -5.0);
    let cylinder_height = 1.0;
    let cylinder_radius = 0.6;
    let cylinder_material = Rc::new(Lambertian::new(Colour::new(0.8, 0.0, 0.0))); // Example material, replace with actual material type

    let cylinder = Cylinder::new(
        cylinder_center,
        cylinder_radius,
        cylinder_height,
        cylinder_material
    );

    // Add the cylinder to the world
    world.add(Rc::new(cylinder));

    //world = HittableList::from(Rc::new(BvhNode::from_list(world)));

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
    cam.lookfrom = Point3::new(2.0, 3.0, -9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    cam.brightness = 1.0;

    cam.render(&world)
}
