use crate::image_utils::aabb::AABB;
use crate::image_utils::hittable::{HitRecord, Hittable};
use crate::image_utils::ray::Ray;
use crate::utils::vec3::Vec3;
use std::borrow::Borrow;
use std::sync::Arc;

pub struct RotateY {
    pub to_rotate: Arc<dyn Hittable + Send + Sync>,
    pub bbox: Option<AABB>,
    pub sin_theta: f32,
    pub cos_theta: f32,
}

impl RotateY {
    pub fn new(to_rotate: Arc<dyn Hittable + Send + Sync>, angle: f32) -> RotateY {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = to_rotate.bounding_box(0.0, 1.0);
        let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.unwrap().max.x + (1 - i) as f32 * bbox.unwrap().min.x;
                    let y = j as f32 * bbox.unwrap().max.y + (1 - j) as f32 * bbox.unwrap().min.y;
                    let z = k as f32 * bbox.unwrap().max.z + (1 - k) as f32 * bbox.unwrap().min.z;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        RotateY {
            to_rotate,
            bbox,
            sin_theta,
            cos_theta,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];
        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];
        let rotated_ray = Ray::new(origin, direction, ray.time);

        if let Some(mut record) = self.to_rotate.hit(&rotated_ray, t_min, t_max) {
            let mut position = record.position;
            let mut normal = record.normal;
            position[0] = self.cos_theta * record.position[0] + self.sin_theta * record.position[2];
            position[2] =
                -self.sin_theta * record.position[0] + self.cos_theta * record.position[2];
            normal[0] = self.cos_theta * record.normal[0] + self.sin_theta * record.normal[2];
            normal[2] = -self.sin_theta * record.normal[0] + self.cos_theta * record.normal[2];
            record.position = position;
            record.set_face_normal(&rotated_ray, normal);
            Some(record)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.bbox
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(RotateY::new(self.to_rotate.clone_dyn(), 0.0))
    }
}
