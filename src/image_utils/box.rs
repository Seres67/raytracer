use crate::image_utils::aabb::AABB;
use crate::image_utils::hittable::{HitRecord, Hittable, HittableList};
use crate::image_utils::ray::Ray;
use crate::image_utils::rectangle::{XYRect, XZRect, YZRect};
use crate::materials::materials::Material;
use crate::utils::vec3::Vec3;
use std::sync::Arc;

pub struct Box {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: HittableList,
}

impl Box {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<dyn Material + Send + Sync>) -> Box {
        let mut sides = HittableList::new();
        sides.add(Arc::new(XYRect::new(
            material.clone(),
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
        )));
        sides.add(Arc::new(XYRect::new(
            material.clone(),
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
        )));
        sides.add(Arc::new(XZRect::new(
            material.clone(),
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
        )));
        sides.add(Arc::new(XZRect::new(
            material.clone(),
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
        )));
        sides.add(Arc::new(YZRect::new(
            material.clone(),
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
        )));
        sides.add(Arc::new(YZRect::new(
            material.clone(),
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
        )));
        Box {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hittable for Box {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(Box {
            box_min: self.box_min,
            box_max: self.box_max,
            sides: self.sides.clone(),
        })
    }
}
