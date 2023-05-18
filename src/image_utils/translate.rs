use crate::image_utils::hittable::Hittable;
use crate::utils::vec3::Vec3;
use std::sync::Arc;

pub struct Translate {
    pub to_translate: Arc<dyn Hittable + Send + Sync>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(to_translate: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Translate {
        Translate {
            to_translate,
            offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(
        &self,
        r: &crate::image_utils::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::image_utils::hittable::HitRecord> {
        let moved_r =
            crate::image_utils::ray::Ray::new(r.origin - self.offset, r.direction, r.time);
        if let Some(mut rec) = self.to_translate.hit(&moved_r, t_min, t_max) {
            rec.position = rec.position + self.offset;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<crate::image_utils::aabb::AABB> {
        self.to_translate
            .bounding_box(time0, time1)
            .map(|output_box| {
                crate::image_utils::aabb::AABB::new(
                    output_box.min + self.offset,
                    output_box.max + self.offset,
                )
            })
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(Translate::new(self.to_translate.clone_dyn(), self.offset))
    }
}
