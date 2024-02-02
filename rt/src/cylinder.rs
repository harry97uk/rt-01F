use std::rc::Rc;

use crate::{
    hittable::{ HitRecord, Hittable },
    material::Material,
    ray::Ray,
    vector3::{ dot, Point3, Vector3 },
    interval::Interval,
    aabb::Aabb,
};
pub struct Cylinder {
    center: Point3, // Center of the cylinder
    radius: f64, // Radius of the cylinder
    height: f64, // Height of the cylinder
    m: Rc<dyn Material>, // Material of the cylinder
    bbox: Aabb,
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t: Interval, hit_record: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_height = self.height / 2.0;

        let direction = r.direction();
        let axis_dot_direction = dot(direction, Vector3::new(0.0, 1.0, 0.0));

        // let y = direction.y();
        let dot_y = oc.y();

        let mut roots: Vec<f64> = Vec::new();

        let a = a - axis_dot_direction * axis_dot_direction;
        let b = 2.0 * (dot(oc, direction) - dot_y * axis_dot_direction);
        let c = oc.length_squared() - dot_y * dot_y - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let sqrt_d = discriminant.sqrt();

            let root = (-b - sqrt_d) / (2.0 * a);
            if root < t.max && root > t.min {
                roots.push(root);
            }

            let root = (-b + sqrt_d) / (2.0 * a);
            if root < t.max && root > t.min {
                roots.push(root);
            }
        } else {
            return false;
        }

        let mut hit_occurred = false;
        let mut closest_so_far = t.max;

        for root in roots {
            let hit_point = r.at(root);

            if hit_point.y() > -half_height && hit_point.y() < half_height {
                if root < closest_so_far {
                    hit_occurred = true;
                    closest_so_far = root;
                    hit_record.t = root;
                    hit_record.p = hit_point;
                    let outward_normal = (hit_record.p - self.center) / self.radius;
                    hit_record.normal = outward_normal;
                    hit_record.mat = self.m.clone();
                }
            }
        }

        hit_occurred
    }

    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.bbox
    }
}

impl Cylinder {
    pub fn new(center: Point3, radius: f64, height: f64, m: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            height,
            m,
            bbox: Aabb::default(),
        }
    }

    fn calculate_outward_normal(&self, point: Point3) -> Vector3 {
        let distance = point.y() - self.center.y();

        if distance > 0.0 {
            Vector3::new(0.0, 1.0, 0.0)
        } else if distance < 0.0 {
            Vector3::new(0.0, -1.0, 0.0)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
}
