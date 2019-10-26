extern crate num;

use std::ops::{Add, Sub, Neg, Mul, Div};

const EPSILON: f64 = 0.00001;

fn equal_f64(a: f64, b: f64) -> bool {
    let diff = a - b;
    if num::abs(diff) < EPSILON {
        return true;
    }
    return false;
}

#[derive(Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 1.0,
    }
}

pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 0.0,
    }
}

impl Eq for Tuple {}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equal_f64(self.w, other.w) && equal_f64(self.x, other.x) && equal_f64(self.y, other.y) && equal_f64(self.z, other.z)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Tuple{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Tuple{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Tuple{
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Tuple{
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}


impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
}

impl Tuple {
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::{new_point, new_vector, Tuple};

    #[test]
    fn new_vector_is_vector() {
        let x = new_vector(4.3, -4.2, 3.1);
        assert_eq!(true, x.is_vector());
        assert_eq!(false, x.is_point());
        assert_eq!(0.0, x.w);
    }

    #[test]
    fn new_point_is_point() {
        let x = new_point(4.3, -4.2, 3.1);
        assert_eq!(false, x.is_vector());
        assert_eq!(true, x.is_point());
        assert_eq!(1.0, x.w);
    }

    #[test]
    fn tuples_equal() {
        let x = new_point(4.3, -4.2, 3.1);
        let y = new_point(4.3, -4.2, 3.1);
        assert_eq!(x, y);
    }

    #[test]
    fn tuples_not_equal() {
        let x = new_point(4.3, -4.2, 3.1);
        let y = new_vector(4.3, -4.2, 3.1);
        assert_ne!(x, y);

        let y = new_point(4.3, -4.2, 3.1);
        let z = new_point(4.3, -4.2, 3.2);
        assert_ne!(y, z);
    }

    #[test]
    fn add_tuples() {
        let a = Tuple{x:3.0, y:-2.0, z:5.0, w:1.0};
        let b = Tuple{x:-2.0, y:3.0, z:1.0, w:0.0};
        let expected = Tuple{x: 1.0, y:1.0, z: 6.0, w: 1.0};

        let res = a + b;
        assert_eq!(res, expected);
        assert_eq!(true, res.is_point());
    }

    #[test]
    fn sub_two_points() {
        let a = new_point(3.0, 2.0, 1.0);
        let b = new_point(5.0, 6.0, 7.0);
        let expected = new_vector(-2.0, -4.0, -6.0);

        let res = a - b;
        assert_eq!(res, expected);
        assert_eq!(true, res.is_vector());
    }

    #[test]
    fn sub_point_vector() {
        let a = new_point(3.0, 2.0, 1.0);
        let b = new_vector(5.0, 6.0, 7.0);
        let expected = new_point(-2.0, -4.0, -6.0);

        let res = a - b;
        assert_eq!(res, expected);
        assert_eq!(true, res.is_point());
    }

    #[test]
    fn sub_two_vectors(){
        let a = new_vector(3.0, 2.0, 1.0);
        let b = new_vector(5.0, 6.0, 7.0);
        let expected = new_vector(-2.0, -4.0, -6.0);

        let res = a - b;
        assert_eq!(res, expected);
        assert_eq!(true, res.is_vector());
    }

    #[test]
    fn neg_tuple(){
        let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        let expected = Tuple{x: -1.0, y: 2.0, z: -3.0, w: 4.0};
        assert_eq!(-a, expected);
    }

    #[test]
    fn mul_tuple(){
        let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        let expected = Tuple{x: 3.5, y: -7.0, z: 10.5, w: -14.0};
        assert_eq!(a * 3.5, expected);
    }

    #[test]
    fn div_tuple(){
        let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        let expected = Tuple{x: 0.5, y: -1.0, z: 1.5, w: -2.0};
        assert_eq!(a / 2.0, expected);
    }
}
