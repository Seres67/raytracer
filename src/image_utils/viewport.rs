use crate::utils::vec3::Vec3;

pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub focal_length: f32,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Viewport
{
    pub fn new(width: u32, height: u32, focal_length: f32, origin: Vec3, horizontal: Vec3, vertical: Vec3, lower_left_corner: Vec3) -> Viewport {
        Viewport { width, height, focal_length, origin, horizontal, vertical, lower_left_corner }
    }
}