use crate::image_utils::image::Image;
use crate::image_utils::viewport::Viewport;
use crate::utils::ray::Ray;
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
    let width = 400.0 / aspect_ratio;
    let mut image = Image::new(400, width as u32, 255);
    let viewport = Viewport::new((aspect_ratio * 2.0) as u32, 2, 1.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(aspect_ratio * 2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Vec3::new(-aspect_ratio, -1.0, -1.0));
    for (mut index, mut pixel) in image.data.iter_mut().enumerate() {
        let x = index as u32 % image.width;
        let y = index as u32 / image.width;

        let u = x as f32 / (image.width - 1) as f32;
        let v = y as f32 / (image.height - 1) as f32;
        let ray = Ray::new(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, viewport.lower_left_corner + u * viewport.horizontal + v * viewport.vertical);
        let color = ray.color();
        pixel.r = (color.x * 255.999) as u8;
        pixel.g = (color.y * 255.999) as u8;
        pixel.b = (color.z * 255.999) as u8;


        index += 1;
        if index as u32 / image.width != y {
            println!("Remaining lines: {}", image.height - y);
        }
    }
    image.write_to_file("test.ppm");
}
