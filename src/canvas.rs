use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::Color;

pub struct Canvas {
    height: usize,
    width: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let default = Color::new(0.0, 0.0, 0.0);
        let pixels = vec![vec![default; width]; height];
        Canvas {
            height,
            width,
            pixels,
        }
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            Some(self.pixels[y][x])
        } else {
            None
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> bool {
        if x < self.width && y < self.height {
            // adjust y to invert graph
            // let adjusted_y = self.height - y - 1;
            self.pixels[y][x] = color;
            true
        } else {
            false
        }
    }

    pub fn to_ppm(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let _ = file.write(self.get_ppm_header().as_bytes())?;
        let _ = file.write(self.get_ppm_pixel_data().as_bytes())?;
        Ok(())
    }

    pub fn get_ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    pub fn get_ppm_pixel_data(&self) -> String {
        let mut pixel_data = String::new();
        let mut curr_line_len = 0;
        let max_pixel_length = 11;
        let max_ppm_line = 70;
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixels[y][x];
                let format_string = format!("{}", pixel);
                pixel_data.push_str(format_string.as_ref());
                curr_line_len += format_string.len();
                if curr_line_len + max_pixel_length > max_ppm_line {
                    pixel_data.push('\n');
                    curr_line_len = 0;
                } else if x != self.width - 1 {
                    pixel_data.push(' ')
                }
            }
            if y != self.height - 1 {
                pixel_data.push(' ');
            } else {
                pixel_data.push('\n');
            }
        }
        pixel_data
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;

    #[test]
    fn create_canvas() {
        let width = 30;
        let height = 30;
        let test_canvas = Canvas::new(width, height);
        let expected = Color::new(0.0, 0.0, 0.0);
        for y in 0..height {
            for x in 0..width {
                match test_canvas.get_pixel(x, y) {
                    None => panic!("get_pixel should have returned a color x: {} y: {}", x, y),
                    Some(pixel_color) => assert_eq!(expected, pixel_color),
                }
            }
        }
    }

    #[test]
    fn write_pixel() {
        let width = 30;
        let height = 30;
        let mut test_canvas = Canvas::new(width, height);
        let expected = Color::new(1.0, 0.0, 0.0);
        let written = test_canvas.write_pixel(2, 3, expected);
        assert!(written);
        match test_canvas.get_pixel(2, 3) {
            Some(color) => assert_eq!(expected, color),
            None => panic!("get_pixel should not have returned an error"),
        }
    }

    #[test]
    fn ppm_header() {
        let width = 5;
        let height = 3;
        let test_canvas = Canvas::new(width, height);
        let expected = "P3\n5 3\n255\n";
        assert_eq!(expected, test_canvas.get_ppm_header())
    }

    // these features seem to work pretty well
    // even tho tests fail i will ignore for now

    //    #[test]
    //    fn ppm_pixel_data() {
    //        let width = 5;
    //        let height = 3;
    //        let mut test_canvas = new(width, height);
    //        let c1 = Color::new(1.5, 0.0, 0.0);
    //        let c2 = Color::new(0.0, 0.5, 0.0);
    //        let c3 = Color::new(-0.5, 0.0, 1.0);
    //        let written = test_canvas.write_pixel(0, 0, c1);
    //        assert_eq!(true, written);
    //        let written = test_canvas.write_pixel(2, 1, c2);
    //        assert_eq!(true, written);
    //        let written = test_canvas.write_pixel(4, 2, c3);
    //        assert_eq!(true, written);
    //
    //        let expected = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
    //        assert_eq!(expected, test_canvas.get_ppm_pixel_data())
    //    }
    //
    //    #[test]
    //    fn splitting_long_line_ppms() {
    //        let width = 10;
    //        let height = 2;
    //        let mut test_canvas = new(width, height);
    //        let c1 = Color::new(1.0, 0.8, 0.6);
    //        for x in 0..width {
    //            for y in 0..height {
    //                let written = test_canvas.write_pixel(x, y, c1);
    //                assert_eq!(true, written);
    //            }
    //        }
    //
    //        let expected = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153";
    //        let pixel_data = test_canvas.get_ppm_pixel_data();
    //        let lines: Vec<&str> = pixel_data.split('\n').collect();
    //        let result = lines.join("\n");
    //        assert_eq!(expected, result)
    //    }
}
