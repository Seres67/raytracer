use crate::image_utils::aabb::AABB;
use crate::image_utils::hittable::{HitRecord, Hittable};
use crate::image_utils::ray::Ray;
use crate::materials::materials::Material;
use crate::utils::vec3::Vec3;
use std::sync::Arc;

pub struct XYRect {
    material: Arc<dyn Material + Send + Sync>,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XYRect {
    pub fn new(
        material: Arc<dyn Material + Send + Sync>,
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
    ) -> XYRect {
        XYRect {
            material,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut record = HitRecord::new(ray.at(t), self.material.clone(), t);
        record.u = (x - self.x0) / (self.x1 - self.x0);
        record.v = (y - self.y0) / (self.y1 - self.y0);
        record.set_face_normal(ray, Vec3::new(0.0, 0.0, 1.0));
        Some(record)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(XYRect::new(
            self.material.clone(),
            self.x0,
            self.x1,
            self.y0,
            self.y1,
            self.k,
        ))
    }
}

pub struct XZRect {
    material: Arc<dyn Material + Send + Sync>,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl XZRect {
    pub fn new(
        material: Arc<dyn Material + Send + Sync>,
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
    ) -> XZRect {
        XZRect {
            material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut record = HitRecord::new(ray.at(t), self.material.clone(), t);
        record.u = (x - self.x0) / (self.x1 - self.x0);
        record.v = (z - self.z0) / (self.z1 - self.z0);
        record.set_face_normal(ray, Vec3::new(0.0, 1.0, 0.0));
        Some(record)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(XZRect::new(
            self.material.clone(),
            self.x0,
            self.x1,
            self.z0,
            self.z1,
            self.k,
        ))
    }
}

pub struct YZRect {
    material: Arc<dyn Material + Send + Sync>,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl YZRect {
    pub fn new(
        material: Arc<dyn Material + Send + Sync>,
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        k: f32,
    ) -> YZRect {
        YZRect {
            material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut record = HitRecord::new(ray.at(t), self.material.clone(), t);
        record.u = (y - self.y0) / (self.y1 - self.y0);
        record.v = (z - self.z0) / (self.z1 - self.z0);
        record.set_face_normal(ray, Vec3::new(1.0, 0.0, 0.0));
        Some(record)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(YZRect::new(
            self.material.clone(),
            self.y0,
            self.y1,
            self.z0,
            self.z1,
            self.k,
        ))
    }
}
