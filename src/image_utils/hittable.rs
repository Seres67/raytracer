use std::sync::Arc;
use crate::image_utils::ray::Ray;
use crate::materials::materials::Material;
use crate::utils::vec3::Vec3;

pub trait Hittable
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync>;
}

pub struct HitRecord
{
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord
{
    pub fn new(position: Vec3, material: Arc<dyn Material>, t: f32) -> HitRecord {
        HitRecord { position, normal: Vec3::new(0.0, 0.0, 0.0), material, t, front_face: false }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub struct HittableList
{
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList
{
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_anything = Some(hit_record);
            }
        }
        hit_anything
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self.clone())
    }
}

impl Clone for HittableList
{
    fn clone(&self) -> Self {
        let mut objects = Vec::new();
        for object in &self.objects {
            objects.push(object.clone_dyn());
        }
        HittableList { objects }
    }
}