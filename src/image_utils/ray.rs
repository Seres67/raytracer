use crate::image_utils::hittable::Hittable;
use crate::utils::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn color(
        &self,
        background: Vec3,
        world: &(dyn Hittable + Send + Sync),
        depth: i32,
    ) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        let record = world.hit(self, 0.001, f32::INFINITY);
        if let Some(..) = record {
            let record = record.unwrap();
            let emitted = record.material.emitted(record.u, record.v, record.position);
            let option = record.material.scatter(self, &record);
            if let Some(..) = option {
                let (scattered, attenuation) = option.unwrap();
                emitted + attenuation * scattered.color(background, world, depth - 1)
            } else {
                emitted
            }
        } else {
            background
        }
    }
}
