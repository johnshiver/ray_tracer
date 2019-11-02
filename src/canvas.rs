use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::{new_color, Color};

pub struct Canvas {
    height: usize,
    width: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        let pos = x * y;
        if pos <= self.width * self.height {
            Some(self.pixels[y][x])
        } else {
            None
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> bool {
        let pos = x * y;
        if pos <= self.width * self.height {
            self.pixels[y][x] = color;
            true
        } else {
            false
        }
    }

    pub fn to_ppm(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        file.write(self.get_ppm_header().as_bytes())?;
        Ok(())
    }

    pub fn get_ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.height, self.width)
    }
}

pub fn new(height: usize, width: usize) -> Canvas {
    let default = new_color(0.0, 0.0, 0.0);
    let size = height * width;
    let pixels = vec![vec![default; width]; height];

    Canvas {
        height,
        width,
        pixels,
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::new;
    use crate::color::new_color;

    #[test]
    fn create_canvas() {
        let height = 30;
        let width = 30;
        let test_canvas = new(height, width);
        let expected = new_color(0.0, 0.0, 0.0);
        for y in 0..height - 1 {
            for x in 0..width - 1 {
                match test_canvas.get_pixel(x, y) {
                    None => assert!(
                        false,
                        "get_pixel should have returned a color x: {} y: {}",
                        x, y
                    ),
                    Some(pixel_color) => assert_eq!(expected, pixel_color),
                }
            }
        }
    }

    #[test]
    fn write_pixel() {
        let height = 30;
        let width = 30;
        let mut test_canvas = new(height, width);
        let expected = new_color(1.0, 0.0, 0.0);
        test_canvas.set_pixel(2, 3, expected);
        match test_canvas.get_pixel(2, 3) {
            Some(color) => assert_eq!(expected, color),
            None => assert!(false, "get_pixel should not have returned an error"),
        }
    }

    #[test]
    fn ppm_header() {
        let height = 5;
        let width = 3;
        let test_canvas = new(height, width);
        let expected = "P3\n5 3\n255\n";
        assert_eq!(expected, test_canvas.get_ppm_header())
    }
}
