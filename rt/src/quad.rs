use std::{ rc::Rc, cell::RefCell };

use crate::{
    vector3::{ Point3, Vector3, cross, unit_vector, dot },
    material::Material,
    aabb::Aabb,
    hittable::{ Hittable, HitRecord },
    hittable_list::HittableList,
};

pub struct Plane {
    q: Point3,
    u: Vector3,
    v: Vector3,
    mat: Rc<dyn Material>,
    bbox: Aabb,
    normal: Vector3,
    d: f64,
    w: Vector3,
}

impl Plane {
    pub fn new(q: Point3, u: Vector3, v: Vector3, m: Rc<dyn Material>) -> Self {
        let n = cross(u, v);
        let normal = unit_vector(n);
        let d = dot(normal, q);
        let w = n / dot(n, n);

        Plane { q, u, v, mat: m, bbox: Aabb::from_points(q, q + u + v).pad(), normal, d, w }
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if a < 0.0 || 1.0 < a || b < 0.0 || 1.0 < b {
            return false;
        }

        rec.u = a;
        rec.v = b;
        return true;
    }
}

impl Hittable for Plane {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord
    ) -> bool {
        let denom = dot(self.normal, r.direction());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(self.w, cross(planar_hitpt_vector, self.v));
        let beta = dot(self.w, cross(self.u, planar_hitpt_vector));

        if !Plane::is_interior(alpha, beta, rec) {
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        return true;
    }

    fn bounding_box(&self) -> Aabb {
        return self.bbox;
    }
}

pub fn cuboid(a: Point3, b: Point3, mat: Rc<dyn Material>) -> Rc<HittableList> {
    // Returns the 3D box (six sides) that contains the two opposite vertices a & b.

    let mut sides = HittableList::new();

    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vector3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vector3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vector3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Rc::new(Plane::new(Point3::new(min.x(), min.y(), max.z()), dx, dy, mat.clone()))); // front
    sides.add(Rc::new(Plane::new(Point3::new(max.x(), min.y(), max.z()), -dz, dy, mat.clone()))); // right
    sides.add(Rc::new(Plane::new(Point3::new(max.x(), min.y(), min.z()), -dx, dy, mat.clone()))); // back
    sides.add(Rc::new(Plane::new(Point3::new(min.x(), min.y(), min.z()), dz, dy, mat.clone()))); // left
    sides.add(Rc::new(Plane::new(Point3::new(min.x(), max.y(), max.z()), dx, -dz, mat.clone()))); // top
    sides.add(Rc::new(Plane::new(Point3::new(min.x(), min.y(), min.z()), dx, dz, mat))); // bottom

    return Rc::new(sides);
}
