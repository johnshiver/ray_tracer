use crate::color::{new_color, Color};
use num::range;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct Canvas {
    height: u32,
    width: u32,
    pixels: Vec<Color>,
}

impl Canvas {
    fn buffer_size(&self) -> u32 {
        self.height * self.width
    }

    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = x * (y * self.width);
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        match self.get_offset(x, y) {
            Some(offset) => Some(self.pixels[offset]),
            None => None,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.pixels[offset] = color;
                true
            }
            None => false,
        }
    }

    pub fn write_image(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write(header.as_bytes())?;
        file.write(&self.pixels)?;
        Ok(())
    }
}

pub fn new(height: u32, width: u32) -> Canvas {
    let default = new_color(0.0, 0.0, 0.0);
    let size = height * width;
    let mut pixels = vec![default; size as usize];

    Canvas {
        height,
        width,
        pixels,
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::new;
    use crate::canvas::HEIGHT;
    use crate::canvas::WIDTH;
    use crate::color::new_color;

    #[test]
    fn create_canvas() {
        let height = 30;
        let width = 30;
        let test_canvas = new(height, width);
        let expected = new_color(0.0, 0.0, 0.0);
        for i in 0..height - 1 {
            for j in 0..width - 1 {
                assert_eq!(test_canvas.get_pixel(i, j), expected);
            }
        }
    }

    #[test]
    fn write_pixel() {
        let height = 30;
        let width = 30;
        let mut test_canvas = new(height, width);
        let expected = new_color(1.0, 0.0, 0.0);
        test_canvas.write_pixel(2, 3, expected);
        assert_eq!(test_canvas.get_pixel(2, 3), expected);
    }
}
