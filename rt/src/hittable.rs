use std::rc::Rc;

use crate::Ray;
use crate::aabb::Aabb;
use crate::colour::Colour;
use crate::material::{ Material, Lambertian };
use crate::vector3::{ Point3, Vector3, dot };
use crate::interval::Interval;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vector3::default(),
            mat: Rc::new(Lambertian::new(Colour::default())),
            t: 0.0,
            front_face: false,
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}
