use crate::image_utils::ray::Ray;
use crate::utils::vec3::Vec3;
use std::mem::swap;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub(crate) min: Vec3,
    pub(crate) max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub(crate) fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub(crate) fn surrounding_box(&self, second_box: AABB) -> AABB {
        let small = Vec3::new(
            self.min.x.min(second_box.min.x),
            self.min.y.min(second_box.min.y),
            self.min.z.min(second_box.min.z),
        );
        let big = Vec3::new(
            self.max.x.max(second_box.max.x),
            self.max.y.max(second_box.max.y),
            self.max.z.max(second_box.max.z),
        );
        AABB::new(small, big)
    }
}
