use std::sync::Arc;
use crate::utils::vec3::Vec3;

pub trait Texture
{
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

pub struct SolidColor
{
    color_value: Vec3,
}

impl SolidColor
{
    pub(crate) fn new(color_value: Vec3) -> SolidColor
    {
        SolidColor { color_value }
    }
}

impl Texture for SolidColor
{
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3
    {
        self.color_value
    }
}

pub struct CheckerTexture
{
    odd: Arc<dyn Texture + Send + Sync>,
    even: Arc<dyn Texture + Send + Sync>,
}

impl CheckerTexture
{
    pub fn new(odd: Arc<dyn Texture + Send + Sync>, even: Arc<dyn Texture + Send + Sync>) -> CheckerTexture
    {
        CheckerTexture { odd, even }
    }

    pub fn from_color(odd: Vec3, even: Vec3) -> CheckerTexture
    {
        CheckerTexture { odd: Arc::new(SolidColor::new(odd)), even: Arc::new(SolidColor::new(even)) }
    }
}

impl Texture for CheckerTexture
{
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0
        {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}