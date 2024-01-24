use std::rc::Rc;

use crate::{ vector3::{ Point3, dot }, hittable::Hittable, interval::Interval, material::Material };

pub struct Sphere {
    centre: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere { centre, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: Interval,
        rec: &mut crate::hittable::HitRecord
    ) -> bool {
        //The following is a simplified quadratic formula
        let oc = r.origin() - self.centre;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.centre) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
}
