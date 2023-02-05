use std::sync::Arc;
use crate::image_utils::aabb::AABB;
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

    fn get_sphere_uv(&self, point: Vec3) -> (f32, f32) {
        let phi = point.z.atan2(point.x);
        let theta = point.y.asin();
        (1.0 - (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI), (theta + std::f32::consts::PI / 2.0) / std::f32::consts::PI)
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
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut record = HitRecord::new(ray.at(root), self.material.clone(), root);
        let outward_normal = (record.position - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        let (u, v) = self.get_sphere_uv(outward_normal);
        record.u = u;
        record.v = v;
        Some(record)
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB::new(self.center - Vec3::new(self.radius, self.radius, self.radius), self.center + Vec3::new(self.radius, self.radius, self.radius)))
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(Sphere::new(self.center, self.radius, self.material.clone()))
    }
}

pub struct MovingSphere
{
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Arc<dyn Material + Send + Sync>,
}

impl MovingSphere
{
    pub fn new(center0: Vec3, center1: Vec3, time0: f32, time1: f32, radius: f32, material: Arc<dyn Material + Send + Sync>) -> MovingSphere {
        MovingSphere { center0, center1, time0, time1, radius, material }
    }

    fn center(&self, time: f32) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut record = HitRecord::new(ray.at(root), self.material.clone(), root);
        let outward_normal = (record.position - self.center(ray.time)) / self.radius;
        record.set_face_normal(ray, outward_normal);
        Some(record)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        let box0 = AABB::new(self.center(time0) - Vec3::new(self.radius, self.radius, self.radius), self.center(time0) + Vec3::new(self.radius, self.radius, self.radius));
        let box1 = AABB::new(self.center(time1) - Vec3::new(self.radius, self.radius, self.radius), self.center(time1) + Vec3::new(self.radius, self.radius, self.radius));
        Some(box0.surrounding_box(box1))
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(MovingSphere::new(self.center0, self.center1, self.time0, self.time1, self.radius, self.material.clone()))
    }
}
