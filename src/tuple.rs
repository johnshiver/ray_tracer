extern crate num;

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

const EPSILON: f64 = 0.00001;

fn equal_f64(a: f64, b: f64) -> bool {
    let diff = a - b;
    if num::abs(diff) < EPSILON {
        return true;
    }
    false
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let tuple_type = if self.is_vector() { "vector" } else { "point" }.to_string();
        write!(
            f,
            "{} x: {} y: {} z: {}",
            tuple_type, self.x, self.y, self.z
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
}

impl Tuple {
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl Tuple {
    pub fn is_unit_vector(&self) -> Result<bool, TupleNotVectorError> {
        let is_vec_res = self.is_vector();
        if !is_vec_res {
            return Err(TupleNotVectorError::new(
                "is_unit_vector: tuple is not a vector",
            ));
        }

        let res = magnitude(self.clone());
        match res {
            Ok(m) => Ok(m == 1.0),
            Err(e) => Err(e),
        }
    }
}

// Errors --------------------------------------------------
#[derive(Debug)]
pub struct TupleNotVectorError {
    details: String,
}

impl TupleNotVectorError {
    fn new(msg: &str) -> TupleNotVectorError {
        TupleNotVectorError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for TupleNotVectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for TupleNotVectorError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl PartialEq for TupleNotVectorError {
    fn eq(&self, other: &Self) -> bool {
        self.details == other.details
    }
}

// Functions ------------------------------------------------------------------------

pub fn magnitude(vector: Tuple) -> Result<f64, TupleNotVectorError> {
    if !vector.is_vector() {
        return Err(TupleNotVectorError::new(
            "tuple passed to magnitude must be a vector",
        ));
    }
    let magnitude =
        (vector.x.powi(2) + vector.y.powi(2) + vector.z.powi(2) + vector.w.powi(2)).sqrt();
    Ok(magnitude)
}

pub fn normalize(vector: Tuple) -> Result<Tuple, TupleNotVectorError> {
    if !vector.is_vector() {
        return Err(TupleNotVectorError::new(
            "tuple passed to normalize must be a vector",
        ));
    }
    let vector_mag = magnitude(vector).unwrap();
    Ok(Tuple {
        x: vector.x / vector_mag,
        y: vector.y / vector_mag,
        z: vector.z / vector_mag,
        w: vector.w / vector_mag,
    })
}

// Performs dot product on two vectors
//
// The smaller the dot product, the larger the angle between the vectors
// A doc product of 1 means the vectors are identical, and a dot product of -1
// means they point in opposite directions.
//
// If the vectors are unit vectors, the dot product is actually the cosine of the angles between them
fn dot(vec_a: Tuple, vec_b: Tuple) -> Result<f64, TupleNotVectorError> {
    if !(vec_a.is_vector() && vec_b.is_vector()) {
        return Err(TupleNotVectorError::new(
            "dot: both vec_a and vec_b must be vectors",
        ));
    }
    Ok(vec_a.x * vec_b.x + vec_a.y * vec_b.y + vec_a.z * vec_b.z + vec_a.w + vec_b.w)
}

// Returns a new vector that is perpendicular to both of the original vectors
fn cross(vec_a: Tuple, vec_b: Tuple) -> Result<Tuple, TupleNotVectorError> {
    if !(vec_a.is_vector() && vec_b.is_vector()) {
        return Err(TupleNotVectorError::new(
            "dot: both vec_a and vec_b must be vectors",
        ));
    }
    Ok(new_vector(
        vec_a.y * vec_b.z - vec_a.z * vec_b.y,
        vec_a.z * vec_b.x - vec_a.x * vec_b.z,
        vec_a.x * vec_b.y - vec_a.y * vec_b.x,
    ))
}

// Tests --------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::tuple::{
        cross, dot, magnitude, new_point, new_vector, normalize, Tuple, TupleNotVectorError,
    };

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
    fn sub_two_vectors() {
        let a = new_vector(3.0, 2.0, 1.0);
        let b = new_vector(5.0, 6.0, 7.0);
        let expected = new_vector(-2.0, -4.0, -6.0);

        let res = a - b;
        assert_eq!(res, expected);
        assert_eq!(true, res.is_vector());
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
        let a = new_vector(1.0, 0.0, 0.0);
        let expected = 1.0;
        let res = magnitude(a);
        match res {
            Ok(m) => assert_eq!(m, expected),
            Err(e) => assert!(
                false,
                "error calculating magnitude when there should be none"
            ),
        }

        let b = new_vector(0.0, 1.0, 0.0);
        let expected = 1.0;
        let res = magnitude(b);
        match res {
            Ok(m) => assert_eq!(m, expected),
            Err(e) => assert!(
                false,
                "error calculating magnitude when there should be none"
            ),
        }

        let c = new_vector(0.0, 0.0, 1.0);
        let expected = 1.0;
        let res = magnitude(c);
        match res {
            Ok(m) => assert_eq!(m, expected),
            Err(e) => assert!(
                false,
                "error calculating magnitude when there should be none"
            ),
        }

        let d = new_vector(1.0, 2.0, 3.0);
        let expected = 14.0_f64.sqrt();
        let res = magnitude(d);
        match res {
            Ok(m) => assert_eq!(m, expected),
            Err(e) => assert!(
                false,
                "error calculating magnitude when there should be none"
            ),
        }

        let e = new_vector(-1.0, -2.0, -3.0);
        let expected = 14.0_f64.sqrt();
        let res = magnitude(e);
        match res {
            Ok(m) => assert_eq!(m, expected),
            Err(e) => assert!(
                false,
                "error calculating magnitude when there should be none"
            ),
        }
    }

    #[test]
    fn magnitude_error() {
        let e = new_point(-1.0, -2.0, -3.0);
        let expected = TupleNotVectorError::new("tuple passed to magnitude must be a vector");
        let res = magnitude(e);
        match res {
            Ok(m) => assert!(false, "this should have been an error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn is_unit_vector_cases() {
        let a = new_vector(1.0, 0.0, 0.0);
        let expected = 1.0;
        let res = magnitude(a);
        match res {
            Ok(m) => assert_eq!(m, expected),
            Err(e) => assert!(
                false,
                "error calculating magnitude when there should be none"
            ),
        }

        let is_uv_res = a.is_unit_vector();
        match is_uv_res {
            Ok(m) => assert_eq!(m, true),
            Err(e) => assert!(
                false,
                "error calculating is_unit_vector when there should be none"
            ),
        }
    }

    #[test]
    fn normalize_success() {
        let a = new_vector(4.0, 0.0, 0.0);
        let expected = new_vector(1.0, 0.0, 0.0);

        let normalize_res = normalize(a);
        match normalize_res {
            Ok(nv) => assert_eq!(nv, expected),
            Err(_e) => assert!(
                false,
                "error calculating normalize_res when there should be none"
            ),
        }

        let a = new_vector(1.0, 2.0, 3.0);
        let expected = new_vector(0.26726, 0.53452, 0.80178);
        let normalize_res = normalize(a);
        match normalize_res {
            Ok(nv) => assert_eq!(nv, expected),
            Err(_e) => assert!(
                false,
                "error calculating normalize_res when there should be none"
            ),
        }

        let a = new_vector(1.0, 2.0, 3.0);
        let expected = new_vector(0.26726, 0.53452, 0.80178);
        let normalize_res = normalize(a);
        let normalized_vector = normalize_res.unwrap();
        let expected_mag = 1.0;
        assert_eq!(expected_mag, magnitude(normalized_vector).unwrap());
    }

    #[test]
    fn normalize_failure() {
        let e = new_point(-1.0, -2.0, -3.0);
        let expected = TupleNotVectorError::new("tuple passed to normalize must be a vector");
        let res = normalize(e);
        match res {
            Ok(m) => assert!(false, "this should have been an error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn dot_product_success() {
        let vec_a = new_vector(1.0, 2.0, 3.0);
        let vec_b = new_vector(2.0, 3.0, 4.0);
        let expected = 20.0;

        assert_eq!(dot(vec_a, vec_b).unwrap(), expected);
    }

    #[test]
    fn cross_product_success() {
        let vec_a = new_vector(1.0, 2.0, 3.0);
        let vec_b = new_vector(2.0, 3.0, 4.0);
        let expected_a_b = new_vector(-1.0, 2.0, -1.0);
        let expected_b_a = new_vector(1.0, -2.0, 1.0);

        assert_eq!(cross(vec_a, vec_b).unwrap(), expected_a_b);
        assert_eq!(cross(vec_b, vec_a).unwrap(), expected_b_a);
    }
}
