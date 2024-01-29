use std::{ rc::Rc, f64::consts::PI };

use crate::{
    vector3::{ Point3, dot, Vector3 },
    hittable::Hittable,
    interval::Interval,
    material::Material,
    aabb::Aabb,
};

pub struct Sphere {
    centre: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let rvec = Vector3::new(radius, radius, radius);
        let bbox = Aabb::from_points(centre - rvec, centre + rvec);
        Sphere { centre, radius, mat, bbox }
    }

    fn get_sphere_uv(p: Point3, u: &mut f64, v: &mut f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y()).acos();
        let phi = -p.z().atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
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
        Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone();

        return true;
    }

    fn bounding_box(&self) -> Aabb {
        return self.bbox;
    }
}
