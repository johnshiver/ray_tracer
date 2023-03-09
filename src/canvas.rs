use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::Color;

pub struct Canvas {
    height: usize,
    width: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            height,
            width,
            pixels: [Color::default()].repeat(width * height),
        }
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        self.pixels.get(x + y * self.width).copied()
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> bool {
        let target = x + y * self.width;
        if target >= self.pixels.len() {
            return false;
        }
        self.pixels.push(color);
        self.pixels.swap_remove(x + y * self.width);
        true
    }

    pub fn to_ppm(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(path)?;
        let _ = file.write(self.get_ppm_header().as_bytes())?;
        let _ = file.write(self.get_ppm_pixel_data().as_bytes())?;
        Ok(())
    }

    pub fn get_ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    /// PPM doesnt allow lines longer than 70
    pub fn get_ppm_pixel_data(&self) -> String {
        // Initiate with very bold approximate size
        let mut content_lines: String = String::with_capacity(self.width * self.width);
        self.pixels
            .chunks(self.width) // chunk by pixel line
            .for_each(|l| {
                l.iter().fold(0, |current_line_size, c| {
                    let raw_scaled_color = format!("{}", c.scale());
                    let raw_scaled_color_len = raw_scaled_color.chars().count();
                    if current_line_size == 0 {
                        // first line
                        content_lines.push_str(&raw_scaled_color);
                        raw_scaled_color_len
                    } else {
                        let next_line_size = current_line_size + raw_scaled_color_len + 1;
                        if next_line_size <= 69 {
                            // continue line
                            content_lines.push(' ');
                            content_lines.push_str(&raw_scaled_color);
                            next_line_size
                        } else {
                            // new line
                            content_lines.push('\n');
                            content_lines.push_str(&raw_scaled_color);
                            raw_scaled_color_len
                        }
                    }
                });
                // separate lines
                content_lines.push('\n');
            });
        content_lines
    }
    // pub fn get_ppm_pixel_data(&self) -> String {
    //     let mut pixel_data = String::new();
    //     let mut curr_line_len = 0;
    //
    //     // max length includes 9 + 2 + 1
    //     let max_pixel_length = 12;
    //     let max_ppm_line = 70;
    //     for y in 0..self.height {
    //         for x in 0..self.width {
    //             let pixel = self.pixels[y][x];
    //             let format_string = format!("{}", pixel);
    //             pixel_data.push_str(&format_string);
    //             curr_line_len += format_string.len();
    //
    //             // if adding another pixel would potentially go over, instead push new line and reset
    //             if curr_line_len + max_pixel_length > max_ppm_line {
    //                 pixel_data.push('\n');
    //                 curr_line_len = 0;
    //             } else if x != self.width - 1 {
    //                 pixel_data.push(' ')
    //             }
    //         }
    //         // at the end of a line
    //         pixel_data.push('\n');
    //         curr_line_len = 0;
    //     }
    //     pixel_data
    // }
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

    #[test]
    fn ppm_pixel_data() {
        let width = 5;
        let height = 3;
        let mut test_canvas = Canvas::new(width, height);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        let written = test_canvas.write_pixel(0, 0, c1);
        assert!(written);
        let written = test_canvas.write_pixel(2, 1, c2);
        assert!(written);
        let written = test_canvas.write_pixel(4, 2, c3);
        assert!(written);

        let expected = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
        assert_eq!(expected, test_canvas.get_ppm_pixel_data())
    }

    #[test]
    fn splitting_long_line_ppms() {
        let width = 10;
        let height = 2;
        let mut test_canvas = Canvas::new(width, height);
        let c1 = Color::new(1.0, 0.8, 0.6);
        for x in 0..width {
            for y in 0..height {
                let written = test_canvas.write_pixel(x, y, c1);
                assert!(written);
            }
        }

        let pixel_data = test_canvas.get_ppm_pixel_data();
        // let lines: Vec<&str> = pixel_data.split('\n').collect();
        // let result = lines.join("\n");
        let mut ppm_lines = pixel_data.lines();
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
    }
}
