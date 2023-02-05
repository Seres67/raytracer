use crate::image_utils::aabb::AABB;
use crate::image_utils::hittable::{HitRecord, Hittable};
use crate::image_utils::ray::Ray;
use crate::materials::materials::{Isotropic, Material};
use crate::utils::vec3::Vec3;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Send + Sync>,
    phase_function: Arc<dyn Material + Send + Sync>,
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable + Send + Sync>,
        phase_function: Arc<dyn Material + Send + Sync>,
        density: f32,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function,
            neg_inv_density: -1.0 / density,
        }
    }

    pub fn new_from_color(
        boundary: Arc<dyn Hittable + Send + Sync>,
        color: Vec3,
        density: f32,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function: Arc::new(Isotropic::new(color)),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut record) = self
            .boundary
            .hit(ray, -std::f32::INFINITY, std::f32::INFINITY)
        {
            if let Some(mut record2) = self
                .boundary
                .hit(ray, record.t + 0.0001, std::f32::INFINITY)
            {
                if record.t < t_min {
                    record.t = t_min;
                }
                if record2.t > t_max {
                    record2.t = t_max;
                }
                if record.t >= record2.t {
                    return None;
                }
                if record.t < 0.0 {
                    record.t = 0.0;
                }
                let ray_length = ray.direction.length();
                let distance_inside_boundary = (record2.t - record.t) * ray_length;
                let hit_distance = self.neg_inv_density * rand::random::<f32>().ln();
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = record.t + hit_distance / ray_length;
                let mut record = HitRecord::new(ray.at(t), self.phase_function.clone(), t);
                record.set_face_normal(ray, record.normal);
                Some(record)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(ConstantMedium {
            boundary: self.boundary.clone_dyn(),
            phase_function: self.phase_function.clone(),
            neg_inv_density: self.neg_inv_density,
        })
    }
}
