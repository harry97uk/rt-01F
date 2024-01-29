use std::{ rc::Rc, ops::{ DerefMut, Deref } };

use crate::{ hittable::{ Hittable, HitRecord }, interval::Interval, aabb::Aabb };

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: Aabb,
}

impl<'a> HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![], bbox: Aabb::default() }
    }

    pub fn from(object: Rc<dyn Hittable>) -> Self {
        let bbox = Aabb::from_boxes(Aabb::default(), object.bounding_box());
        HittableList { objects: [object].to_vec(), bbox }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = Aabb::from_boxes(self.bbox, object.bounding_box());
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

    fn bounding_box(&self) -> Aabb {
        return self.bbox;
    }
}
