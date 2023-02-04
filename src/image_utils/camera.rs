use crate::image_utils::ray::Ray;
use crate::utils::vec3::Vec3;

pub struct Camera
{
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera
{
    pub fn new(look_from: Vec3, look_at: Vec3, view_up: Vec3, vertical_fov: f32, aspect_ratio: f32, aperture: f32, focus_distance: f32) -> Camera {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (look_from - look_at).unit_vector();
        let u = view_up.cross(w).unit_vector();
        let v = w.cross(u);
        let origin = look_from;
        let horizontal = viewport_width * u * focus_distance;
        let vertical = viewport_height * v * focus_distance;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;
        Camera { origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius: aperture / 2.0 }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}

impl Clone for Camera
{
    fn clone(&self) -> Camera {
        Camera { origin: self.origin, lower_left_corner: self.lower_left_corner, horizontal: self.horizontal, vertical: self.vertical, u: self.u, v: self.v, w: self.w, lens_radius: self.lens_radius }
    }
}