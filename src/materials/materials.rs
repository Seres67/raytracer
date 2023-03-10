use crate::image_utils::hittable::HitRecord;
use crate::image_utils::ray::Ray;
use crate::image_utils::texture::{SolidColor, Texture};
use crate::utils::random_double;
use crate::utils::vec3::Vec3;
use std::sync::Arc;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    pub texture: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Lambertian {
        Lambertian {
            texture: Arc::new(SolidColor::new(color)),
        }
    }

    pub fn from_texture(texture: Arc<dyn Texture + Send + Sync>) -> Lambertian {
        Lambertian { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.position, scatter_direction, _ray.time);
        Some((
            scattered,
            self.texture
                .value(hit_record.u, hit_record.v, hit_record.position),
        ))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.unit_vector().reflect(hit_record.normal);
        let scattered = Ray::new(
            hit_record.position,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray.time,
        );
        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }

    pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, self.refraction_index) > random_double()
        {
            unit_direction.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, refraction_ratio)
        };
        let ray = Ray::new(hit_record.position, direction, ray.time);
        Some((ray, Vec3::new(1.0, 1.0, 1.0)))
    }
}

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn new(color: Vec3) -> DiffuseLight {
        DiffuseLight {
            emit: Arc::new(SolidColor::new(color)),
        }
    }

    pub fn from_texture(texture: Arc<dyn Texture + Send + Sync>) -> DiffuseLight {
        DiffuseLight { emit: texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture + Send + Sync>,
}

impl Isotropic {
    pub fn new(color: Vec3) -> Isotropic {
        Isotropic {
            albedo: Arc::new(SolidColor::new(color)),
        }
    }

    pub fn from_texture(texture: Arc<dyn Texture + Send + Sync>) -> Isotropic {
        Isotropic { albedo: texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(hit_record.position, Vec3::random_in_unit_sphere(), ray.time);
        Some((
            scattered,
            self.albedo
                .value(hit_record.u, hit_record.v, hit_record.position),
        ))
    }
}
