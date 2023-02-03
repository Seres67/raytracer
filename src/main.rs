use std::rc::Rc;
use crate::image_utils::image::Image;
use crate::image_utils::camera::Camera;
use crate::image_utils::hittable::HittableList;
use crate::image_utils::sphere::Sphere;
use crate::materials::materials::{Dielectric, Lambertian, Metal};
use crate::utils::random_double;
use crate::utils::vec3::Vec3;

mod image_utils;
mod utils;
mod materials;

fn main() {
    // let mut image = Image::new(256, 256, 255);
    // for (mut index, mut pixel) in image.data.iter_mut().enumerate() {
    //     let x = index as u32 % image.width;
    //     let y = index as u32 / image.width;
    //     pixel.r = (x as f32 / (256 - 1) as f32 * 255.999) as u8;
    //     pixel.g = ((y as f32 / (256 - 1) as f32) * 255.999) as u8;
    //     pixel.b = (0.25 * 255.999) as u8;
    //     index += 1;
    //     if index as u32 / image.width != y {
    //         println!("Remaining lines: {}", image.height - y);
    //     }
    // }
    let aspect_ratio = 16.0 / 9.0;
    let height = 400.0 / aspect_ratio;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let scale = 1.0 / samples_per_pixel as f32;

    let mut image = Image::new(400, height as u32, 255);
    let mut list = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));
    let material_left = Rc::new(Dielectric::new(1.5));

    let sphere_ground = Sphere::new(Vec3::new(0.0, 100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_right = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_left = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right);

    list.add(Box::new(sphere_ground));
    list.add(Box::new(sphere_center));
    list.add(Box::new(sphere_left));
    list.add(Box::new(sphere_right));

    let camera = Camera::new();

    for (mut index, mut pixel) in image.data.iter_mut().enumerate() {
        let x = index as u32 % image.width;
        let y = index as u32 / image.width;


        let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u = (x as f32 + random_double()) / (image.width - 1) as f32;
            let v = (y as f32 + random_double()) / (image.height - 1) as f32;
            let ray = camera.get_ray(u, v);
            let color = ray.color(&list, max_depth);
            pixel_color = pixel_color + color;
        }
        let mut r = pixel_color.x * scale;
        r = r.sqrt();
        r = r.clamp(0.0, 0.999);
        pixel.r = (r * 256.0) as u8;
        let mut g = pixel_color.y * scale;
        g = g.sqrt();
        g = g.clamp(0.0, 0.999);
        pixel.g = (g * 256.0) as u8;
        let mut b = pixel_color.z * scale;
        b = b.sqrt();
        b = b.clamp(0.0, 0.999);
        pixel.b = (b * 256.0) as u8;

        index += 1;
        if index as u32 / image.width != y {
            println!("Remaining lines: {}", image.height - y);
        }
    }
    image.write_to_file("test2.ppm");
}
