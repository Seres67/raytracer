use crate::utils::random_double_range;
use crate::utils::vec3::Vec3;

pub struct Perlin {
    ranfloat: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranvec = Vec::new();
        for _ in 0..256 {
            ranvec.push(Vec3::random_range(-1.0, 1.0).unit_vector());
        }
        Perlin {
            ranfloat: ranvec,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, point: Vec3) -> f32 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();
        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut c = [[[Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let perm_x = self.perm_x[((i + di) & 255) as usize];
                    let perm_y = self.perm_y[((j + dj) & 255) as usize];
                    let perm_z = self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] =
                        self.ranfloat[perm_x as usize ^ perm_y as usize ^ perm_z as usize];
                }
            }
        }
        Self::trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut accumulator = 0.0;
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_x = if i == 0 { 1.0 - uu } else { uu };
                    let weight_y = if j == 0 { 1.0 - vv } else { vv };
                    let weight_z = if k == 0 { 1.0 - ww } else { ww };
                    let weight_c = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accumulator += weight_x
                        * weight_y
                        * weight_z
                        * c[i as usize][j as usize][k as usize].dot(weight_c);
                }
            }
        }
        accumulator
    }

    pub fn turbulence(&self, point: Vec3, depth: i32) -> f32 {
        let mut accumulator = 0.0;
        let mut weight = 1.0;
        let mut temp_point = point;
        for _ in 0..depth {
            accumulator += weight * self.noise(temp_point);
            weight *= 0.5;
            temp_point = 2.0 * temp_point;
        }
        accumulator.abs()
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::new();
        for i in 0..256 {
            p.push(i);
        }
        Perlin::permute(&mut p, 256);
        p
    }

    fn permute(p: &mut [i32], n: usize) {
        for i in (0..n).rev() {
            let target = random_double_range(0.0, i as f32) as i32;
            p.swap(i, target as usize);
        }
    }
}
