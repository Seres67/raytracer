use crate::image_utils::aabb::AABB;
use crate::image_utils::ray::Ray;
use crate::materials::materials::Material;
use crate::utils::vec3::Vec3;
use std::sync::Arc;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB>;
    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync>;

    fn box_compare(&self, b: &Arc<dyn Hittable + Send + Sync>, axis: usize) -> bool {
        let aabb = self.bounding_box(0.0, 0.0).unwrap();
        let babb = b.bounding_box(0.0, 0.0).unwrap();
        aabb.min[axis] < babb.min[axis]
    }

    fn box_x_compare(&self, b: &Arc<dyn Hittable + Send + Sync>) -> bool {
        self.box_compare(b, 0)
    }

    fn box_y_compare(&self, b: &Arc<dyn Hittable + Send + Sync>) -> bool {
        self.box_compare(b, 1)
    }

    fn box_z_compare(&self, b: &Arc<dyn Hittable + Send + Sync>) -> bool {
        self.box_compare(b, 2)
    }
}

pub struct HitRecord {
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(position: Vec3, material: Arc<dyn Material>, t: f32) -> HitRecord {
        HitRecord {
            position,
            normal: Vec3::new(0.0, 0.0, 0.0),
            material,
            t,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
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

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }
        let mut output_box: Option<AABB> = None;
        for object in &self.objects {
            if let Some(temp_box) = object.bounding_box(time0, time1) {
                output_box = match output_box {
                    Some(output_box) => Some(output_box.surrounding_box(temp_box)),
                    None => Some(temp_box),
                };
            } else {
                return None;
            }
        }
        output_box
    }

    fn clone_dyn(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self.clone())
    }
}

impl Clone for HittableList {
    fn clone(&self) -> Self {
        let mut objects = Vec::new();
        for object in &self.objects {
            objects.push(object.clone_dyn());
        }
        HittableList { objects }
    }
}
