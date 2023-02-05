use crate::image_utils::camera::Camera;
use crate::image_utils::hittable::HittableList;
use crate::image_utils::image::{Image, Pixel};
use crate::image_utils::sphere::{MovingSphere, Sphere};
use crate::image_utils::texture::CheckerTexture;
use crate::materials::materials::{Dielectric, Lambertian, Material, Metal};
use crate::utils::vec3::Vec3;
use crate::utils::{random_double, random_double_range};
use std::sync::{Arc, Mutex};
use std::thread;

mod image_utils;
mod materials;
mod utils;

const NUMBER_OF_THREADS: usize = 7;

#[allow(dead_code)]
fn random_scene() -> HittableList {
    let mut list = HittableList::new();
    // let material_ground = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let checker = Arc::new(CheckerTexture::from_color(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let sphere = Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(checker)),
    );
    list.add(Arc::new(sphere));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3::new(
                a as f32 + 0.9 * random_double(),
                0.2,
                b as f32 + 0.9 * random_double(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material + Send + Sync>;
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 1.0), 0.0);
                    let sphere = MovingSphere::new(center, center2, 0.0, 1.0, 0.2, material);
                    list.add(Arc::new(sphere));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    material = Arc::new(Metal::new(albedo, fuzz));
                    let sphere = Sphere::new(center, 0.2, material);
                    list.add(Arc::new(sphere));
                } else {
                    material = Arc::new(Dielectric::new(1.5));
                    let sphere = Sphere::new(center, 0.2, material);
                    list.add(Arc::new(sphere));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let sphere = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1);
    list.add(Arc::new(sphere));

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let sphere = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2);
    list.add(Arc::new(sphere));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    let sphere = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3);
    list.add(Arc::new(sphere));
    list
}

fn basic_scene() -> HittableList {
    let mut list = HittableList::new();

    let material_center = Arc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

    let material_ground = Arc::new(CheckerTexture::from_color(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    let sphere_ground = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::from_texture(material_ground)),
    );
    let sphere_center = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_right = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right);

    list.add(Arc::new(sphere_ground));
    list.add(Arc::new(sphere_center));
    list.add(Arc::new(sphere_left));
    list.add(Arc::new(sphere_right));
    list
}

fn run(id: usize, width: u32, height: u32, camera: &Camera, list: &HittableList) -> Vec<Pixel> {
    let samples_per_pixel = 500;
    let max_depth = 50;
    let scale = 1.0 / samples_per_pixel as f32;
    let mut index = id * width as usize;
    let total_lines = (height as f32 / NUMBER_OF_THREADS as f32).round() as usize;
    let mut current_line = 0;
    let mut out = Vec::new();
    out.resize(total_lines * width as usize, Pixel::new(0, 0, 0));
    println!("Thread {id} is currently processing line {current_line} out of {total_lines}.");
    for (_, pixel) in out.iter_mut().enumerate() {
        let x = index as u32 % width;
        let y = height - (index as u32 / width) - 1;

        let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u = (x as f32 + random_double()) / (width - 1) as f32;
            let v = (y as f32 + random_double()) / (height - 1) as f32;
            let ray = camera.get_ray(u, v);
            pixel_color = pixel_color + ray.color(list, max_depth);
        }
        pixel.r = ((pixel_color.x * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
        pixel.g = ((pixel_color.y * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
        pixel.b = ((pixel_color.z * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
        index += 1;
        if index % width as usize == 0 {
            index += width as usize * (NUMBER_OF_THREADS - 1);
            current_line += 1;
            println!(
                "Thread {id} is currently processing line {current_line} out of {total_lines}."
            );
        }
    }
    out
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width: f32 = 1920.0;

    let mut image = Image::new(width as u32, (width / aspect_ratio) as u32, 255);
    let list = Arc::new(basic_scene());

    let lookfrom = Vec3::new(3.0, 0.5, 1.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 4.2;
    let aperture = 0.1;

    let camera = Arc::new(Camera::new(
        lookfrom,
        lookat,
        view_up,
        30.0,
        aspect_ratio,
        aperture,
        distance_to_focus,
        0.0,
        1.0,
    ));

    let pixels_out: Arc<Mutex<Vec<Vec<Pixel>>>> =
        Arc::new(Mutex::new(vec![vec![]; NUMBER_OF_THREADS]));
    let mut threads = vec![];
    for i in 0..NUMBER_OF_THREADS {
        let image = image.clone();
        let camera = camera.clone();
        let pixels_out = pixels_out.clone();
        let list = list.clone();
        threads.push(thread::spawn(move || {
            let out = run(i, image.width, image.height, &camera, &list);
            pixels_out.lock().unwrap()[i] = out;
            println!("Thread {i} finished.");
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }

    let pixels_out = pixels_out.lock().unwrap();
    let iterations = pixels_out[0].len() as u32 / image.width + 1;
    for i in 0..iterations {
        println!(
            "Writing line {} of {}.",
            i,
            pixels_out[0].len() as u32 / image.width
        );
        for j in 0..NUMBER_OF_THREADS {
            let start = (i * image.width) as usize;
            let mut end = start + image.width as usize;
            if end > pixels_out[j].len() {
                end = pixels_out[j].len();
            }
            let src = &pixels_out[j][start..end];
            image.data.extend(src);
        }
    }
    image.write_to_file("test12.ppm");
}
