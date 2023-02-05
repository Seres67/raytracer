use crate::image_utils::aabb::AABB;
use crate::image_utils::hittable::{HitRecord, Hittable, HittableList};
use crate::image_utils::ray::Ray;
use std::sync::Arc;

pub struct BVHNode {
    pub left: Arc<dyn Hittable + Send + Sync>,
    pub right: Arc<dyn Hittable + Send + Sync>,
    pub box_: AABB,
}

impl BVHNode {
    pub fn new(
        left: Arc<dyn Hittable + Send + Sync>,
        right: Arc<dyn Hittable + Send + Sync>,
        box_: AABB,
    ) -> BVHNode {
        BVHNode { left, right, box_ }
    }

    pub fn from_list(
        list: HittableList,
        time0: f32,
        time1: f32,
    ) -> Arc<dyn Hittable + Send + Sync> {
        let mut objects = list.objects;
        let axis = 3 * rand::random::<usize>();
        objects.sort_by(|a, b| {
            let box_a = a.bounding_box(time0, time1).unwrap();
            let box_b = b.bounding_box(time0, time1).unwrap();
            if box_a.min[axis] < box_b.min[axis] {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        let left = if objects.len() == 1 {
            objects[0].clone_dyn()
        } else {
            BVHNode::from_list(
                HittableList {
                    objects: objects[0..objects.len() / 2].to_vec(),
                },
                time0,
                time1,
            )
        };
        let right = if objects.len() == 1 {
            objects[0].clone_dyn()
        } else {
            BVHNode::from_list(
                HittableList {
                    objects: objects[objects.len() / 2..].to_vec(),
                },
                time0,
                time1,
            )
        };
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();
        Arc::new(BVHNode::new(
            left,
            right,
            box_left.surrounding_box(box_right),
        ))
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.box_.hit(ray, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(ray, t_min, t_max);
        if hit_left.as_ref().is_some() && hit_right.as_ref().is_some() {
            if hit_left.as_ref().unwrap().t < hit_right.as_ref().unwrap().t {
                return hit_left;
            }
            return hit_right;
        }
        if hit_left.is_some() {
            return hit_left;
        }
        hit_right
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(self.box_)
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(BVHNode::new(
            self.left.clone(),
            self.right.clone(),
            self.box_,
        ))
    }
}
