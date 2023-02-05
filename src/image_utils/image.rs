use std::fmt::Display;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub max_value: u8,
    pub data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: u32, height: u32, max_value: u8) -> Image {
        Image {
            width,
            height,
            max_value,
            data: Vec::with_capacity((width * height) as usize),
        }
    }

    pub fn write_to_file(&self, path: &str) {
        let file = File::create(path);
        let mut file = match file {
            Ok(file) => file,
            Err(e) => {
                println!("Couldn't create file: {e}");
                return;
            }
        };
        let mut output = format!(
            "P3\n{} {}\n{}\n",
            self.width,
            self.data.len() as u32 / self.width,
            self.max_value
        );

        let mut index = 0;
        for pixel in &self.data {
            output.push_str(pixel.to_string().as_str());
            output.push(' ');
            index += 1;
            if index % self.width == 0 {
                output.push('\n');
            }
        }
        file.write_all(output.as_bytes())
            .expect("Couldn't write to file");
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "P3\n{} {}\n{}\n",
            self.width, self.height, self.max_value
        )?;
        for (mut index, pixel) in self.data.iter().enumerate() {
            write!(f, "{pixel} ")?;
            index += 1;
            if index as u32 % self.width == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
