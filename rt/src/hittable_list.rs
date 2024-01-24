use std::rc::Rc;

use crate::{ hittable::{ Hittable, HitRecord }, interval::Interval };

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl<'a> HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn from(object: Rc<dyn Hittable>) -> Self {
        HittableList { objects: [object].to_vec() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl<'a> Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, mut rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        return hit_anything;
    }
}
