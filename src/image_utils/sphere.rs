use std::sync::Arc;
use crate::image_utils::hittable::{HitRecord, Hittable};
use crate::image_utils::ray::Ray;
use crate::materials::materials::Material;
use crate::utils::vec3::Vec3;

pub struct Sphere
{
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Sphere
{
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material + Send + Sync>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();
        let root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut record = HitRecord::new(ray.at(root), self.material.clone(), root);
        let outward_normal = (record.position - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        Some(record)
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(Sphere::new(self.center, self.radius, self.material.clone()))
    }
}
