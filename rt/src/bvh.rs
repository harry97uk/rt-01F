// use std::{ rc::Rc, cmp::Ordering };

// use crate::{
//     hittable::Hittable,
//     aabb::Aabb,
//     hittable_list::HittableList,
//     interval::Interval,
//     rtweekend::random_int,
// };

// pub struct BvhNode {
//     left: Rc<dyn Hittable>,
//     right: Rc<dyn Hittable>,
//     bbox: Aabb,
// }

// impl BvhNode {
//     pub fn new(src_objects: Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
//         let mut new_bvh = BvhNode {
//             left: Rc::new(BvhNode::from_list(HittableList::new())),
//             right: Rc::new(BvhNode::from_list(HittableList::new())),
//             bbox: Aabb::default(),
//         };
//         let mut objects = src_objects; // Create a modifiable array of the source scene objects

//         let axis = random_int(0, 2);
//         let comparator = if axis == 0 {
//             Self::box_x_compare
//         } else if axis == 1 {
//             Self::box_y_compare
//         } else {
//             Self::box_z_compare
//         };

//         let object_span = end - start;

//         if object_span == 1 {
//             new_bvh.left = objects[start].clone();
//             new_bvh.right = objects[start].clone();
//         } else if object_span == 2 {
//             if comparator(objects[start].clone(), objects[start + 1].clone()) == Ordering::Less {
//                 new_bvh.left = objects[start].clone();
//                 new_bvh.right = objects[start + 1].clone();
//             } else {
//                 new_bvh.left = objects[start + 1].clone();
//                 new_bvh.right = objects[start].clone();
//             }
//         } else {
//             objects[start..end].sort_by(|a, b| comparator(a.clone(), b.clone()));

//             let mid = start + object_span / 2;
//             new_bvh.left = Rc::new(BvhNode::new(objects.clone(), start, mid));
//             new_bvh.right = Rc::new(BvhNode::new(objects.clone(), mid, end));
//         }

//         new_bvh.bbox = Aabb::from_boxes(new_bvh.left.bounding_box(), new_bvh.right.bounding_box());

//         new_bvh
//     }

//     pub fn from_list(list: HittableList) -> Self {
//         BvhNode::new(list.objects.clone(), 0, list.objects.len())
//     }

//     fn box_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>, axis_index: i32) -> Ordering {
//         let a_min = a.bounding_box().axis(axis_index).min;
//         let b_min = b.bounding_box().axis(axis_index).min;

//         if a_min < b_min {
//             Ordering::Less
//         } else if a_min > b_min {
//             Ordering::Greater
//         } else {
//             Ordering::Equal
//         }
//     }

//     fn box_x_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
//         return Self::box_compare(a, b, 0);
//     }

//     fn box_y_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
//         return Self::box_compare(a, b, 1);
//     }

//     fn box_z_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
//         return Self::box_compare(a, b, 2);
//     }
// }

// impl Hittable for BvhNode {
//     fn hit(
//         &self,
//         r: &crate::ray::Ray,
//         ray_t: crate::interval::Interval,
//         rec: &mut crate::hittable::HitRecord
//     ) -> bool {
//         if !self.bbox.hit(r, ray_t) {
//             return false;
//         }

//         let hit_left = self.left.hit(r, ray_t, rec);
//         let hit_right = self.right.hit(
//             r,
//             Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
//             rec
//         );

//         return hit_left || hit_right;
//     }

//     fn bounding_box(&self) -> Aabb {
//         self.bbox
//     }
// }
