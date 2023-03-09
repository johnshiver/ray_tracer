use std::fmt::{Error, Formatter};
use std::ops::{Add, Mul, Sub};

use crate::tuple::{Point, Tuple};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    components: Tuple,
}

impl Color {
    /// Returns a new Color
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color {
            components: Point::new_point(red, green, blue),
        }
    }

    pub fn default() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn red(&self) -> f64 {
        self.components.x
    }
    pub fn green(&self) -> f64 {
        self.components.y
    }
    pub fn blue(&self) -> f64 {
        self.components.z
    }

    pub fn scale(self) -> Color {
        Color::new(
            scale_color_val(self.red()),
            scale_color_val(self.green()),
            scale_color_val(self.blue()),
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let new_components = self.components - other.components;
        Color::new(new_components.x, new_components.y, new_components.z)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let new_components = self.components + other.components;
        Color::new(new_components.x, new_components.y, new_components.z)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        let new_components = self.components * scalar;
        Color::new(new_components.x, new_components.y, new_components.z)
    }
}

impl Mul for Color {
    type Output = Self;

    /// Blends two colors together
    ///
    /// This method of blending two colors works by multiplying corresponding components
    /// of each color to form a new color. It’s technically called the Hadamard product (or Schur product),
    /// but it doesn’t really matter what you call it. It just needs to produce a new color
    /// where the new red component is the product of the red components of the other colors,
    /// and so on for blue and green.
    fn mul(self, other: Self) -> Self {
        Color::new(
            self.red() * other.red(),
            self.green() * other.green(),
            self.blue() * other.blue(),
        )
    }
}

impl Eq for Color {}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {} {}", self.red(), self.green(), self.blue(),)
    }
}

///
///
/// # Arguments
///
/// * `val`:
///
/// returns: f64
///
/// # Examples
///
/// ```
///
/// ```
fn scale_color_val(val: f64) -> f64 {
    let x: f64 = val * 255.0;

    if x >= 255.0 {
        255.0
    } else if x <= 0.0 {
        0.0
    } else {
        x.ceil()
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn create_color_success() {
        let test_color = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(test_color.red(), -0.5);
        assert_eq!(test_color.green(), 0.4);
        assert_eq!(test_color.blue(), 1.7);
    }

    #[test]
    fn add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(1.6, 0.7, 1.0);

        let res = c1 + c2;
        assert_eq!(expected, res);
    }

    #[test]
    fn sub_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(0.2, 0.5, 0.5);

        let res = c1 - c2;
        assert_eq!(expected, res);
    }

    #[test]
    fn multi_color_by_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let expected = Color::new(0.4, 0.6, 0.8);

        let res = c1 * 2.0;
        assert_eq!(expected, res);
    }

    #[test]
    fn multi_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let expected = Color::new(0.9, 0.2, 0.04);

        let res = c1 * c2;
        assert_eq!(expected, res);
    }
}
