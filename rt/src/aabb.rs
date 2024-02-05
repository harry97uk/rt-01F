use crate::{ interval::Interval, vector3::Point3 };

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn default() -> Self {
        Aabb { x: Interval::empty(), y: Interval::empty(), z: Interval::empty() }
    }
    pub fn new(ix: Interval, iy: Interval, iz: Interval) -> Self {
        Aabb { x: ix, y: iy, z: iz }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.
        let x = Interval::new(f64::min(a.x(), b.x()), f64::max(a.x(), b.x()));
        let y = Interval::new(f64::min(a.y(), b.y()), f64::max(a.y(), b.y()));
        let z = Interval::new(f64::min(a.z(), b.z()), f64::max(a.z(), b.z()));

        Aabb { x, y, z }
    }

    pub fn from_boxes(box0: Aabb, box1: Aabb) -> Self {
        let x = Interval::from_intervals(box0.x, box1.x);
        let y = Interval::from_intervals(box0.y, box1.y);
        let z = Interval::from_intervals(box0.z, box1.z);

        Aabb { x, y, z }
    }

    pub fn pad(&self) -> Self {
        // Return an AABB that has no side narrower than some delta, padding if necessary.
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta { self.x } else { self.x.expand(delta) };
        let new_y = if self.y.size() >= delta { self.y } else { self.y.expand(delta) };
        let new_z = if self.z.size() >= delta { self.z } else { self.z.expand(delta) };

        return Aabb::new(new_x, new_y, new_z);
    }

    // pub fn axis(&self, n: i32) -> Interval {
    //     if n == 1 {
    //         return self.y;
    //     }
    //     if n == 2 {
    //         return self.z;
    //     }
    //     return self.x;
    // }

    // pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
    //     for a in 0..3 {
    //         let inv_d = 1.0 / r.direction().index(a);
    //         let orig = r.origin().index(a);

    //         let mut t0 = (self.axis(a as i32).min - orig) * inv_d;
    //         let mut t1 = (self.axis(a as i32).max - orig) * inv_d;

    //         if inv_d < 0.0 {
    //             (t0, t1) = (t1, t0);
    //         }

    //         if t0 > ray_t.min {
    //             ray_t.min = t0;
    //         }
    //         if t1 < ray_t.max {
    //             ray_t.max = t1;
    //         }

    //         if ray_t.max <= ray_t.min {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
}
