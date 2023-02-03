use crate::image_utils::image::Image;
use crate::image_utils::viewport::Viewport;
use crate::image_utils::camera::Camera;
use crate::image_utils::hittable::HittableList;
use crate::image_utils::sphere::Sphere;
use crate::utils::random_double;
use crate::utils::vec3::Vec3;

mod image_utils;
mod utils;

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
    let mut image = Image::new(400, height as u32, 255);
    let mut list = HittableList::new();
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    list.add(Box::new(sphere));
    list.add(Box::new(sphere2));
    let camera = Camera::new();
    for (mut index, mut pixel) in image.data.iter_mut().enumerate() {
        let x = index as u32 % image.width;
        let y = index as u32 / image.width;

        let scale = 1.0 / samples_per_pixel as f32;

        let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u = (x as f32+ random_double()) / (image.width - 1) as f32;
            let v = (y as f32 + random_double()) / (image.height - 1) as f32;
            let ray = camera.get_ray(u, v);
            pixel_color = pixel_color + ray.color(&list);
        }
        pixel.r = ((pixel_color.x * scale).clamp(0.0, 0.999) * 256.0) as u8;
        pixel.g = ((pixel_color.y * scale).clamp(0.0, 0.999) * 256.0) as u8;
        pixel.b = ((pixel_color.z * scale).clamp(0.0, 0.999) * 256.0) as u8;

        index += 1;
        if index as u32 / image.width != y {
            println!("Remaining lines: {}", image.height - y);
        }
    }
    image.write_to_file("test2.ppm");
}
