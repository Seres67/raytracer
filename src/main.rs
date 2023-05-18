use crate::image_utils::bvh::BVHNode;
use crate::image_utils::camera::Camera;
use crate::image_utils::constant_medium::ConstantMedium;
use crate::image_utils::hittable::{Hittable, HittableList};
use crate::image_utils::image::{Image, Pixel};
use crate::image_utils::rectangle::{XYRect, XZRect, YZRect};
use crate::image_utils::rotate::RotateY;
use crate::image_utils::sphere::{MovingSphere, Sphere};
use crate::image_utils::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::image_utils::translate::Translate;
use crate::materials::materials::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
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

fn two_perlin_spheres() -> HittableList {
    let mut list = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new_with_scale(4.0));
    let sphere = Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    );
    let sphere2 = Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    );
    list.add(Arc::new(sphere));
    list.add(Arc::new(sphere2));
    list
}

fn pepega() -> HittableList {
    let mut list = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new("pepega.png"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface);
    list.add(Arc::new(globe));
    list
}

fn cornell_box() -> HittableList {
    let mut list = HittableList::new();

    let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0)));

    let rect1 = YZRect::new(green, 0.0, 555.0, 0.0, 555.0, 555.0);
    let rect2 = YZRect::new(red, 0.0, 555.0, 0.0, 555.0, 0.0);
    let rect3 = XZRect::new(light, 213.0, 343.0, 227.0, 332.0, 554.0);
    let rect4 = XZRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0);
    let rect5 = XZRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0);
    let rect6 = XYRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0);

    let mut box1: Arc<dyn Hittable + Send + Sync> = Arc::new(crate::image_utils::r#box::Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    let mut box2: Arc<dyn Hittable + Send + Sync> = Arc::new(crate::image_utils::r#box::Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    list.add(Arc::new(rect1));
    list.add(Arc::new(rect2));
    list.add(Arc::new(rect3));
    list.add(Arc::new(rect4));
    list.add(Arc::new(rect5));
    list.add(Arc::new(rect6));
    list.add(box1);
    list.add(box2);
    list
}

fn final_scene() -> HittableList {
    let mut list = HittableList::new();
    let mut boxes = HittableList::new();

    let ground = Arc::new(Lambertian::new(Vec3::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as f32 * w;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;
            boxes.add(Arc::new(crate::image_utils::r#box::Box::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    list.add(BVHNode::from_list(boxes, 0.0, 1.0));

    let light = Arc::new(DiffuseLight::new(Vec3::new(7.0, 7.0, 7.0)));
    list.add(Arc::new(XZRect::new(
        light, 123.0, 423.0, 147.0, 412.0, 554.0,
    )));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.1)));
    list.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));
    list.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    list.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    list.add(boundary.clone());
    list.add(Arc::new(ConstantMedium::new_from_color(
        boundary.clone(),
        Vec3::new(0.2, 0.4, 0.9),
        0.2,
    )));
    let boundary2 = Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    list.add(Arc::new(ConstantMedium::new_from_color(
        boundary2.clone(),
        Vec3::new(1.0, 1.0, 1.0),
        0.0001,
    )));

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_material = Arc::new(Lambertian::from_texture(earth_texture));
    list.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));
    let pertext = Arc::new(NoiseTexture::new_with_scale(0.1));
    list.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;

    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::new(
                random_double_range(0.0, 165.0),
                random_double_range(0.0, 165.0),
                random_double_range(0.0, 165.0),
            ),
            10.0,
            white.clone(),
        )));
    }

    list.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(BVHNode::from_list(boxes2, 0.0, 1.0), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    list
}

fn simple_light() -> HittableList {
    let mut list = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new_with_scale(4.0));
    let sphere = Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    );
    let sphere2 = Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    );
    let difflight = Arc::new(DiffuseLight::new(Vec3::new(4.0, 4.0, 4.0)));
    let rect = XYRect::new(difflight, 3.0, 5.0, 1.0, 3.0, -2.0);
    list.add(Arc::new(sphere));
    list.add(Arc::new(sphere2));
    list.add(Arc::new(rect));
    list
}

fn run(id: usize, width: u32, height: u32, camera: &Camera, list: &HittableList) -> Vec<Pixel> {
    let samples_per_pixel = 10000;
    let max_depth = 50;
    let scale = 1.0 / samples_per_pixel as f32;
    let mut index = id * width as usize;
    let total_lines = (height as f32 / NUMBER_OF_THREADS as f32).ceil();
    let max_index = width * total_lines as u32;
    let mut current_line = 0;
    let mut out = Vec::new();
    // let background = Vec3::new(0.0, 0.0, 0.0);
    let background = Vec3::new(0.7, 0.8, 1.0);
    //out.resize((total_lines * width as f32) as usize, Pixel::new(0, 0, 0));
    println!(
        "Thread {id} is currently processing line {} out of {}.",
        current_line + 1,
        total_lines as u32 + 1
    );
    let mut pixel = Pixel::new(0, 0, 0);
    for _ in index..max_index as usize {
        let x = index as u32 % width;
        let y = height - (index as u32 / width) - 1;

        let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u = (x as f32 + random_double()) / (width - 1) as f32;
            let v = (y as f32 + random_double()) / (height - 1) as f32;
            let ray = camera.get_ray(u, v);
            pixel_color = pixel_color + ray.color(background, list, max_depth);
        }
        pixel.r = ((pixel_color.x * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
        pixel.g = ((pixel_color.y * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
        pixel.b = ((pixel_color.z * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
        out.push(pixel);
        index += 1;
        if index % width as usize == 0 {
            index += width as usize * (NUMBER_OF_THREADS - 1);
            current_line += 1;
            println!(
                "Thread {id} is currently processing line {} out of {}.",
                current_line + 1,
                total_lines as u32 + 1
            );
        }
    }
    out
}

fn main() {
    let aspect_ratio = 1.0;
    let width: f32 = 800.0;

    let mut image = Image::new(width as u32, (width / aspect_ratio) as u32, 255);
    let list = Arc::new(final_scene());

    let lookfrom = Vec3::new(478.0, 278.0, -600.0);
    let lookat = Vec3::new(278.0, 278.0, -1.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 4.2;
    let aperture = 0.1;

    let camera = Arc::new(Camera::new(
        lookfrom,
        lookat,
        view_up,
        40.0,
        aspect_ratio,
        aperture,
        distance_to_focus,
        0.0,
        1.0,
    ));
    // let lookfrom = Vec3::new(478.0, 278.0, -600.0);
    // let lookat = Vec3::new(278.0, 278.0, 0.0);
    // let view_up = Vec3::new(0.0, 1.0, 0.0);
    // let distance_to_focus = 4.2;
    // let aperture = 0.1;
    // let camera = Arc::new(Camera::new(
    //     lookfrom,
    //     lookat,
    //     view_up,
    //     40.0,
    //     aspect_ratio,
    //     aperture,
    //     distance_to_focus,
    //     0.0,
    //     1.0,
    // ));

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
    let iterations = image.height / NUMBER_OF_THREADS as u32 + 1;
    // let iterations = pixels_out[NUMBER_OF_THREADS - 1].len() as u32 / image.width + 1;
    let number_of_lines = image.width.checked_div_euclid(NUMBER_OF_THREADS as u32);
    let remaining_pixels = image.width.checked_rem_euclid(NUMBER_OF_THREADS as u32);
    for i in 0..number_of_lines.unwrap() {
        for j in 0..NUMBER_OF_THREADS {
            println!(
                "Writing line {} of {}.",
                (j + 1) + (i as usize * NUMBER_OF_THREADS),
                image.width
            );
            let start = (i * image.width) as usize;
            if start >= pixels_out[j].len() {
                break;
            }
            let mut end = start + image.width as usize;
            if end > pixels_out[j].len() {
                end = pixels_out[j].len();
            }
            let src = &pixels_out[j][start..end];
            src.to_vec().retain(|x| x.r != 0 && x.g != 0 && x.b != 0);
            image.data.extend(src);
        }
    }
    image.write_to_file("test13.ppm");
}
