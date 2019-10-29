use crate::color::{new_color, Color};
use num::range;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub struct Canvas {
    pixels: [[Color; WIDTH]; HEIGHT],
}

impl Canvas {
    fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y][x] = c
    }
}

pub fn new() -> Canvas {
    let default = new_color(0.0, 0.0, 0.0);
    let mut pixels: [[Color; WIDTH]; HEIGHT] = [[default; WIDTH]; HEIGHT];

    for i in 0..HEIGHT - 1 {
        for j in 0..WIDTH - 1 {
            pixels[i][j] = new_color(0.0, 0.0, 0.0);
        }
    }

    Canvas { pixels }
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
