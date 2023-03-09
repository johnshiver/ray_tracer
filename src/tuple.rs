extern crate num;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utils::equal_f64;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub type Point = Tuple;
pub type Vector = Tuple;

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "x: {} y: {} z: {}", self.x, self.y, self.z)
    }
}

impl Eq for Tuple {}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equal_f64(self.w, other.w)
            && equal_f64(self.x, other.x)
            && equal_f64(self.y, other.y)
            && equal_f64(self.z, other.z)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple {
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
        Tuple {
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
        Tuple {
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
        Tuple {
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

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z, w: 0.0 }
    }

    /// The distance represented by a vector is called its magnitude, or length.
    ///
    /// It’s how far you would travel in a straight line if you were to walk from
    /// one end of the vector to the other.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    /// Vectors with magnitude 1 are a `unit vector` and can be useful for certain operations
    pub fn is_unit_vector(&self) -> bool {
        self.magnitude() == 1.0
    }

    /// Normalize creates a unit vector from vector
    ///
    /// Normalization is the process of taking an arbitrary vector and converting it
    /// into a unit vector.
    ///
    /// It will keep your calculations anchored relative to a common scale (the unit vector),
    /// which is pretty important. If you were to skip normalizing your ray vectors or your
    /// surface normals, your calculations would be scaled differently for every ray you cast,
    /// and your scenes would look terrible (if they rendered at all).
    pub fn normalize(&self) -> Vector {
        let vector_mag = self.magnitude();
        Vector::new(
            self.x / vector_mag,
            self.y / vector_mag,
            self.z / vector_mag,
        )
    }

    /// Performs dot product on two vectors returning a scalar value
    ///
    /// The smaller the dot product, the larger the angle between the vectors
    /// A doc product of 1 means the vectors are identical, and a dot product of -1
    /// means they point in opposite directions.
    ///
    /// If the vectors are unit vectors, the dot product is actually the cosine of the angles between them
    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w + v.w
    }

    /// Returns a new vector that is perpendicular to both of the original vectors
    ///
    /// Order matters for the cross product. X cross Y gives you Z, Y cross X gives -Z
    fn cross(&self, vec_b: &Vector) -> Vector {
        Vector::new(
            self.y * vec_b.z - self.z * vec_b.y,
            self.z * vec_b.x - self.x * vec_b.z,
            self.x * vec_b.y - self.y * vec_b.x,
        )
    }
}

impl Point {
    pub fn new_point(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z, w: 1.0 }
    }
}

// Tests --------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::tuple::{Point, Tuple, Vector};

    #[test]
    fn new_vector_is_vector() {
        let x = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(0.0, x.w);
    }

    #[test]
    fn new_point_is_point() {
        let x = Point::new_point(4.3, -4.2, 3.1);
        assert_eq!(1.0, x.w);
    }

    #[test]
    fn tuples_equal() {
        let x = Point::new_point(4.3, -4.2, 3.1);
        let y = Point::new_point(4.3, -4.2, 3.1);
        assert_eq!(x, y);
    }

    #[test]
    fn tuples_not_equal() {
        let x = Point::new_point(4.3, -4.2, 3.1);
        let y = Vector::new(4.3, -4.2, 3.1);
        assert_ne!(x, y);

        let y = Point::new_point(4.3, -4.2, 3.1);
        let z = Point::new_point(4.3, -4.2, 3.2);
        assert_ne!(y, z);
    }

    #[test]
    fn add_tuples() {
        let a = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let b = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        let expected = Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };

        let res = a + b;
        assert_eq!(res, expected);
        assert!(res.is_point());
    }

    #[test]
    fn sub_two_points() {
        let a = Point::new_point(3.0, 2.0, 1.0);
        let b = Point::new_point(5.0, 6.0, 7.0);
        let expected = Vector::new(-2.0, -4.0, -6.0);

        let res = a - b;
        assert_eq!(res, expected);
        assert!(res.is_vector());
    }

    #[test]
    fn sub_point_vector() {
        let a = Point::new_point(3.0, 2.0, 1.0);
        let b = Vector::new(5.0, 6.0, 7.0);
        let expected = Point::new_point(-2.0, -4.0, -6.0);

        let res = a - b;
        assert_eq!(res, expected);
        assert!(res.is_point());
    }

    #[test]
    fn sub_two_vectors() {
        let a = Vector::new(3.0, 2.0, 1.0);
        let b = Vector::new(5.0, 6.0, 7.0);
        let expected = Vector::new(-2.0, -4.0, -6.0);

        let res = a - b;
        assert_eq!(res, expected);
        assert!(res.is_vector());
    }

    #[test]
    fn neg_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let expected = Tuple {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 4.0,
        };
        assert_eq!(-a, expected);
    }

    #[test]
    fn mul_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let expected = Tuple {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0,
        };
        assert_eq!(a * 3.5, expected);
    }

    #[test]
    fn div_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let expected = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert_eq!(a / 2.0, expected);
    }

    #[test]
    fn magnitude_success() {
        struct Test {
            input: Vector,
            expected: f64,
        }
        let tests = vec![
            Test {
                input: Vector::new(1.0, 0.0, 0.0),
                expected: 1.0,
            },
            Test {
                input: Vector::new(0.0, 1.0, 0.0),
                expected: 1.0,
            },
            Test {
                input: Vector::new(0.0, 0.0, 1.0),
                expected: 1.0,
            },
            Test {
                input: Vector::new(1.0, 2.0, 3.0),
                expected: 14.0_f64.sqrt(),
            },
            Test {
                input: Vector::new(-1.0, -2.0, -3.0),
                expected: 14.0_f64.sqrt(),
            },
        ];

        for t in tests {
            assert_eq!(t.input.magnitude(), t.expected);
        }
    }

    #[test]
    fn is_unit_vector_cases() {
        let a = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(a.magnitude(), 1.0);

        let is_uv_res = a.is_unit_vector();
        assert!(is_uv_res);
    }

    #[test]
    fn normalize_success() {
        let a = Vector::new(4.0, 0.0, 0.0);
        let expected = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(a.normalize(), expected);

        let a = Vector::new(1.0, 2.0, 3.0);
        let expected = Vector::new(0.26726, 0.53452, 0.80178);
        assert_eq!(a.normalize(), expected);

        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, a.normalize().magnitude());
    }

    #[test]
    fn dot_product_success() {
        let vec_a = Vector::new(1.0, 2.0, 3.0);
        let vec_b = Vector::new(2.0, 3.0, 4.0);
        let expected = 20.0;
        assert_eq!(vec_a.dot(&vec_b), expected);
    }

    #[test]
    fn cross_product_success() {
        let vec_a = Vector::new(1.0, 2.0, 3.0);
        let vec_b = Vector::new(2.0, 3.0, 4.0);
        let expected_a_b = Vector::new(-1.0, 2.0, -1.0);
        let expected_b_a = Vector::new(1.0, -2.0, 1.0);

        assert_eq!(vec_a.cross(&vec_b), expected_a_b);
        assert_eq!(vec_b.cross(&vec_a), expected_b_a);
    }
}
