pub mod vec3;

pub fn random_double() -> f32
{
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen();
    x
}

pub fn random_double_range(min: f32, max: f32) -> f32
{
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(min..max);
    x
}