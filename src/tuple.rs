extern crate num;

use std::error::Error;
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

type Point = Tuple;
type Vector = Tuple;


pub fn new_point(x: f64, y: f64, z: f64) -> Point {
    Point {
        x,
        y,
        z,
        w: 1.0,
    }
}

pub fn new_vector(x: f64, y: f64, z: f64) -> Vector {
    Vector {
        x,
        y,
        z,
        w: 0.0,
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "x: {} y: {} z: {}",
            self.x, self.y, self.z
        )
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
    pub fn is_unit_vector(&self) -> bool {
        magnitude(self) == 1.0
    }
}

// Errors --------------------------------------------------
#[derive(Debug)]
pub struct TupleTypeError {
    details: String,
}

impl TupleTypeError {
    pub fn new(msg: &str) -> TupleTypeError {
        TupleTypeError {
            details: msg.to_string(),
        }
    }
}

impl Display for TupleTypeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for TupleTypeError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl PartialEq for TupleTypeError {
    fn eq(&self, other: &Self) -> bool {
        self.details == other.details
    }
}

// Functions ------------------------------------------------------------------------

pub fn magnitude(v: &Vector) -> f64 {
    (v.x.powi(2) + v.y.powi(2) + v.z.powi(2) + v.w.powi(2)).sqrt()
}

pub fn normalize(vector: &Vector) -> Vector {
    let vector_mag = magnitude(vector);
    Vector {
        x: vector.x / vector_mag,
        y: vector.y / vector_mag,
        z: vector.z / vector_mag,
        w: vector.w / vector_mag,
    }
}

// Performs dot product on two vectors
//
// The smaller the dot product, the larger the angle between the vectors
// A doc product of 1 means the vectors are identical, and a dot product of -1
// means they point in opposite directions.
//
// If the vectors are unit vectors, the dot product is actually the cosine of the angles between them
pub fn dot(ta: &Vector, tb: &Vector) -> f64 {
    ta.x * tb.x + ta.y * tb.y + ta.z * tb.z + ta.w + tb.w
}

// Returns a new vector that is perpendicular to both of the original vectors
fn cross(vec_a: &Vector, vec_b: &Vector) -> Vector {
    new_vector(
        vec_a.y * vec_b.z - vec_a.z * vec_b.y,
        vec_a.z * vec_b.x - vec_a.x * vec_b.z,
        vec_a.x * vec_b.y - vec_a.y * vec_b.x,
    )
}

// Tests --------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::tuple::{cross, dot, magnitude, new_point, new_vector, normalize, Tuple, Vector};

    #[test]
    fn new_vector_is_vector() {
        let x = new_vector(4.3, -4.2, 3.1);
        assert_eq!(0.0, x.w);
    }

    #[test]
    fn new_point_is_point() {
        let x = new_point(4.3, -4.2, 3.1);
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
        let a = new_point(3.0, 2.0, 1.0);
        let b = new_point(5.0, 6.0, 7.0);
        let expected = new_vector(-2.0, -4.0, -6.0);

        let res = a - b;
        assert_eq!(res, expected);
        assert!(res.is_vector());
    }

    #[test]
    fn sub_point_vector() {
        let a = new_point(3.0, 2.0, 1.0);
        let b = new_vector(5.0, 6.0, 7.0);
        let expected = new_point(-2.0, -4.0, -6.0);

        let res = a - b;
        assert_eq!(res, expected);
        assert!(res.is_point());
    }

    #[test]
    fn sub_two_vectors() {
        let a = new_vector(3.0, 2.0, 1.0);
        let b = new_vector(5.0, 6.0, 7.0);
        let expected = new_vector(-2.0, -4.0, -6.0);

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
            Test { input: new_vector(1.0, 0.0, 0.0), expected: 1.0 },
            Test { input: new_vector(0.0, 1.0, 0.0), expected: 1.0 },
            Test { input: new_vector(0.0, 0.0, 1.0), expected: 1.0 },
            Test { input: new_vector(1.0, 2.0, 3.0), expected: 14.0_f64.sqrt() },
            Test { input: new_vector(-1.0, -2.0, -3.0), expected: 14.0_f64.sqrt() },
        ];

        for t in tests {
            let res = magnitude(&t.input);
            assert_eq!(res, t.expected);
        }
    }


    #[test]
    fn is_unit_vector_cases() {
        let a = new_vector(1.0, 0.0, 0.0);
        let expected = 1.0;
        let res = magnitude(&a);
        assert_eq!(res, expected);

        let is_uv_res = a.is_unit_vector();
        assert!(is_uv_res);
    }

    #[test]
    fn normalize_success() {
        let a = new_vector(4.0, 0.0, 0.0);
        let expected = new_vector(1.0, 0.0, 0.0);

        let normalize_res = normalize(&a);
        assert_eq!(normalize_res, expected);

        let a = new_vector(1.0, 2.0, 3.0);
        let expected = new_vector(0.26726, 0.53452, 0.80178);
        let normalize_res = normalize(&a);
        assert_eq!(normalize_res, expected);

        let a = new_vector(1.0, 2.0, 3.0);
        let normalized_vector = normalize(&a);
        let expected_mag = 1.0;
        assert_eq!(expected_mag, magnitude(&normalized_vector));
    }

    #[test]
    fn dot_product_success() {
        let vec_a = new_vector(1.0, 2.0, 3.0);
        let vec_b = new_vector(2.0, 3.0, 4.0);
        let expected = 20.0;
        assert_eq!(dot(&vec_a, &vec_b), expected);
    }

    #[test]
    fn cross_product_success() {
        let vec_a = new_vector(1.0, 2.0, 3.0);
        let vec_b = new_vector(2.0, 3.0, 4.0);
        let expected_a_b = new_vector(-1.0, 2.0, -1.0);
        let expected_b_a = new_vector(1.0, -2.0, 1.0);

        assert_eq!(cross(&vec_a, &vec_b), expected_a_b);
        assert_eq!(cross(&vec_b, &vec_a), expected_b_a);
    }
}
