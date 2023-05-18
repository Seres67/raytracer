use crate::utils::perlin::Perlin;
use crate::utils::vec3::Vec3;
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    color_value: Vec3,
}

impl SolidColor {
    pub(crate) fn new(color_value: Vec3) -> SolidColor {
        SolidColor { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture + Send + Sync>,
    even: Arc<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new(
        odd: Arc<dyn Texture + Send + Sync>,
        even: Arc<dyn Texture + Send + Sync>,
    ) -> CheckerTexture {
        CheckerTexture { odd, even }
    }

    pub fn from_color(odd: Vec3, even: Vec3) -> CheckerTexture {
        CheckerTexture {
            odd: Arc::new(SolidColor::new(odd)),
            even: Arc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f32,
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: 1.0,
        }
    }

    pub fn new_with_scale(scale: f32) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turbulence(p, 7)).sin())
    }
}

pub struct ImageTexture {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub bytes_per_scanline: u32,
}

const PIXEL_SIZE: u32 = 3;

impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let image = image::open(filename).unwrap();
        ImageTexture {
            data: image.as_bytes().to_vec(),
            width: image.width(),
            height: image.height(),
            bytes_per_scanline: image.width() * PIXEL_SIZE,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let u = if u < 0.0 {
            0.0
        } else if u > 1.0 {
            1.0
        } else {
            u
        };
        let v = if v < 0.0 {
            0.0
        } else if v > 1.0 {
            1.0
        } else {
            v
        };
        let mut i = (u * self.width as f32) as u32;
        let mut j = self.height - 1 - (v * self.height as f32) as u32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        let pixel = &self.data[(j * self.bytes_per_scanline + i * PIXEL_SIZE) as usize..];
        Vec3::new(
            pixel[0] as f32 * 1.0 / 255.0,
            pixel[1] as f32 * 1.0 / 255.0,
            pixel[2] as f32 * 1.0 / 255.0,
        )
    }
}
