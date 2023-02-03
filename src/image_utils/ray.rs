use crate::image_utils::hittable::Hittable;
use crate::utils::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &dyn Hittable, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        if let Some(record) = world.hit(self, 0.001, f32::INFINITY) {
            if let Some((scattered, attenuation)) = record.material.scatter(self, &record) {
                return attenuation * scattered.color(world, depth - 1);
            }
            return Vec3::new(0.0, 0.0, 0.0);
        }
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}