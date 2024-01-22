use crate::Ray;
use crate::vector3::{ Point3, Vector3, dot };

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord { p: Point3::default(), normal: Vector3::default(), t: 0.0, front_face: false }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}
