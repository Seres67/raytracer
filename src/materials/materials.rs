use crate::image_utils::hittable::HitRecord;
use crate::image_utils::ray::Ray;
use crate::utils::random_double;
use crate::utils::vec3::Vec3;

pub trait Material
{
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian
{
    pub albedo: Vec3,
}

impl Lambertian
{
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian
{
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.position, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metal
{
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal
{
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal
{
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.unit_vector().reflect(hit_record.normal);
        let scattered = Ray::new(hit_record.position, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric
{
    pub refraction_index: f32,
}

impl Dielectric
{
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }

    pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric
{
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let refraction_ratio = if hit_record.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, self.refraction_index) > random_double() {
            unit_direction.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, refraction_ratio)
        };
        let ray = Ray::new(hit_record.position, direction);
        Some((ray, Vec3::new(1.0, 1.0, 1.0)))
    }
}