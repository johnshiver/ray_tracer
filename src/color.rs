use crate::tuple::{new_point, Tuple};
use std::fmt::{Error, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    components: Tuple,
}

pub fn new_color(red: f64, green: f64, blue: f64) -> Color {
    Color {
        components: new_point(red, green, blue),
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let new_components = self.components - other.components;
        new_color(new_components.x, new_components.y, new_components.z)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let new_components = self.components + other.components;
        new_color(new_components.x, new_components.y, new_components.z)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        let new_components = self.components * scalar;
        new_color(new_components.x, new_components.y, new_components.z)
    }
}

// consider moving this down to tuple, although it may not make sense (not a natural property)
impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        new_color(
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
        write!(
            f,
            "{} {} {}",
            scale_color_val(self.red()),
            scale_color_val(self.green()),
            scale_color_val(self.blue()),
        )
    }
}

fn scale_color_val(val: f64) -> i16 {
    let x: f64 = val * 255.0;

    if x >= 255.0 {
        255
    } else if x <= 0.0 {
        0
    } else {
        x.ceil() as i16
    }
}

impl Color {
    pub fn red(&self) -> f64 {
        self.components.x
    }
    pub fn green(&self) -> f64 {
        self.components.y
    }
    pub fn blue(&self) -> f64 {
        self.components.z
    }
}

#[cfg(test)]
mod tests {
    use crate::color::new_color;

    #[test]
    fn create_color_success() {
        let test_color = new_color(-0.5, 0.4, 1.7);
        assert_eq!(test_color.red(), -0.5);
        assert_eq!(test_color.green(), 0.4);
        assert_eq!(test_color.blue(), 1.7);
    }

    #[test]
    fn add_colors() {
        let c1 = new_color(0.9, 0.6, 0.75);
        let c2 = new_color(0.7, 0.1, 0.25);
        let expected = new_color(1.6, 0.7, 1.0);

        let res = c1 + c2;
        assert_eq!(expected, res);
    }

    #[test]
    fn sub_colors() {
        let c1 = new_color(0.9, 0.6, 0.75);
        let c2 = new_color(0.7, 0.1, 0.25);
        let expected = new_color(0.2, 0.5, 0.5);

        let res = c1 - c2;
        assert_eq!(expected, res);
    }

    #[test]
    fn multi_color_by_scalar() {
        let c1 = new_color(0.2, 0.3, 0.4);
        let expected = new_color(0.4, 0.6, 0.8);

        let res = c1 * 2.0;
        assert_eq!(expected, res);
    }

    #[test]
    fn multi_colors() {
        let c1 = new_color(1.0, 0.2, 0.4);
        let c2 = new_color(0.9, 1.0, 0.1);
        let expected = new_color(0.9, 0.2, 0.04);

        let res = c1 * c2;
        assert_eq!(expected, res);
    }
}
