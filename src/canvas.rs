use crate::color::{new_color, Color};
use num::range;

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

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {}
}

pub fn new(height: u32, width: u32) -> Canvas {
    let default = new_color(0.0, 0.0, 0.0);
    let size = height * width;
    let mut pixels: vec![default; size];

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
        let test_canvas = new();
        let expected = new_color(0.0, 0.0, 0.0);
        for i in 0..HEIGHT - 1 {
            for j in 0..WIDTH - 1 {
                assert_eq!(test_canvas.pixels[i][j], expected);
            }
        }
    }

    #[test]
    fn write_pixel() {
        let mut test_canvas = new();
        let expected = new_color(1.0, 0.0, 0.0);
        test_canvas.write_pixel(2, 3, expected);
        assert_eq!(test_canvas.pixel_at(2, 3), expected);
    }
}
