use crate::tuple::{new_point, Tuple};
use std::borrow::Borrow;
use std::ops::{Index, Mul};

const EPSILON: f64 = 0.00001;

fn equal_f64(a: f64, b: f64) -> bool {
    let diff = a - b;
    if num::abs(diff) < EPSILON {
        return true;
    }
    false
}

struct MatrixIndex {
    x: usize,
    y: usize,
}

// ----------------------------- 4x4 ------------------------------------
#[derive(Debug)]
pub struct M4x4 {
    matrix: [[f64; 4]; 4],
}

impl Index<&MatrixIndex> for M4x4 {
    type Output = f64;

    fn index(&self, index: &MatrixIndex) -> &Self::Output {
        match index {
            MatrixIndex { x: 0..=3, y: 0..=3 } => self.matrix[index.y][index.x].borrow(),
            _ => &-99.0,
        }
    }
}

impl Eq for M4x4 {}

impl PartialEq for M4x4 {
    fn eq(&self, other: &Self) -> bool {
        for y in 0..3 {
            for x in 0..3 {
                let mi = MatrixIndex { x, y };
                if !(equal_f64(self[&mi], other[&mi])) {
                    return false;
                }
            }
        }
        true
    }
}

impl Mul<M4x4> for M4x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut new_matrix = [[0.0; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                new_matrix[y][x] = cal_index_matrix_multi(self.matrix, other.matrix, x, y);
            }
        }
        new_4x4(new_matrix)
    }
}

impl Mul<Tuple> for M4x4 {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        Tuple {
            x: cal_index_tuple_multi(self.matrix, other, 0),
            y: cal_index_tuple_multi(self.matrix, other, 1),
            z: cal_index_tuple_multi(self.matrix, other, 2),
            w: cal_index_tuple_multi(self.matrix, other, 3),
        }
    }
}

fn cal_index_matrix_multi(m1: [[f64; 4]; 4], m2: [[f64; 4]; 4], x: usize, y: usize) -> f64 {
    // for y 1, x 0 of new matrix
    // line up row 1 for m1 and col 1 for m2
    let row = m1[y];
    let col = [m2[0][x], m2[1][x], m2[2][x], m2[3][x]];

    let mut final_val = 0.0;
    for i in 0..4 {
        final_val += row[i] * col[i]
    }
    final_val
}

fn cal_index_tuple_multi(m1: [[f64; 4]; 4], t: Tuple, r: usize) -> f64 {
    // for y 1, x 0 of new matrix
    // line up row 1 for m1 and col 1 for m2
    let row = m1[r];
    t.x * row[0] + t.y * row[1] + t.z * row[2] + t.w * row[3]
}

pub fn new_4x4(matrix: [[f64; 4]; 4]) -> M4x4 {
    M4x4 { matrix }
}
// ----------------------------- 3x3 ------------------------------------

#[derive(Debug)]
pub struct M3x3 {
    matrix: [[f64; 3]; 3],
}

impl Index<&MatrixIndex> for M3x3 {
    type Output = f64;

    fn index(&self, index: &MatrixIndex) -> &Self::Output {
        match index {
            MatrixIndex { x: 0..=2, y: 0..=2 } => self.matrix[index.y][index.x].borrow(),
            _ => &-99.0,
        }
    }
}

impl Eq for M3x3 {}

impl PartialEq for M3x3 {
    fn eq(&self, other: &Self) -> bool {
        for y in 0..2 {
            for x in 0..2 {
                let mi = MatrixIndex { x, y };
                if !(equal_f64(self[&mi], other[&mi])) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn new_3x3(matrix: [[f64; 3]; 3]) -> M3x3 {
    M3x3 { matrix }
}

// ----------------------------- 2x2 ------------------------------------
#[derive(Debug)]
pub struct M2x2 {
    matrix: [[f64; 2]; 2],
}

impl Index<&MatrixIndex> for M2x2 {
    type Output = f64;

    fn index(&self, index: &MatrixIndex) -> &Self::Output {
        match index {
            MatrixIndex { x: 0..=1, y: 0..=1 } => self.matrix[index.y][index.x].borrow(),
            _ => &-99.0,
        }
    }
}

impl Eq for M2x2 {}

impl PartialEq for M2x2 {
    fn eq(&self, other: &Self) -> bool {
        for y in 0..1 {
            for x in 0..1 {
                let mi = MatrixIndex { x, y };
                if !(equal_f64(self[&mi], other[&mi])) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn new_2x2(matrix: [[f64; 2]; 2]) -> M2x2 {
    M2x2 { matrix }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{new_2x2, new_3x3, new_4x4, MatrixIndex};
    use crate::tuple::{new_point, Tuple};

    #[test]
    fn create_4x4_matrix() {
        let test_matrix = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];
        let test_m4x4 = new_4x4(test_matrix);
        assert_eq!(test_m4x4[&MatrixIndex { x: 0, y: 0 }], 1.0);
        assert_eq!(test_m4x4[&MatrixIndex { x: 3, y: 0 }], 4.0);
        assert_eq!(test_m4x4[&MatrixIndex { x: 0, y: 1 }], 5.5);
        assert_eq!(test_m4x4[&MatrixIndex { x: 2, y: 1 }], 7.5);
        assert_eq!(test_m4x4[&MatrixIndex { x: 2, y: 2 }], 11.0);
        assert_eq!(test_m4x4[&MatrixIndex { x: 0, y: 3 }], 13.5);
        assert_eq!(test_m4x4[&MatrixIndex { x: 2, y: 3 }], 15.5);
    }

    #[test]
    fn compare_4x4_matrices() {
        let m1 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];
        let m2 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];
        assert_eq!(new_4x4(m1), new_4x4(m2));

        let m3 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];
        let m4 = [
            [2.0, 3.0, 3.0, 5.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];

        assert_ne!(new_4x4(m3), new_4x4(m4));
    }

    #[test]
    fn multiply_4x4_matrices() {
        let m1 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let m2 = [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ];
        let expected = [
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ];

        let test_m1 = new_4x4(m1);
        let test_m2 = new_4x4(m2);
        let test_expected = new_4x4(expected);
        assert_eq!(test_m1 * test_m2, test_expected);
    }

    #[test]
    fn multiply_4x4_matrix_tuple() {
        let m1 = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let t1 = new_point(1.0, 2.0, 3.0);
        let expected = Tuple {
            x: 18.0,
            y: 24.0,
            z: 33.0,
            w: 1.0,
        };

        let test_m1 = new_4x4(m1);
        assert_eq!(test_m1 * t1, expected);
    }

    #[test]
    fn create_3x3_matrix() {
        let test_matrix = [[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]];
        let test_m3x3 = new_3x3(test_matrix);
        assert_eq!(test_m3x3[&MatrixIndex { x: 0, y: 0 }], -3.0);
        assert_eq!(test_m3x3[&MatrixIndex { x: 1, y: 1 }], -2.0);
        assert_eq!(test_m3x3[&MatrixIndex { x: 2, y: 2 }], 1.0);
        assert_eq!(test_m3x3[&MatrixIndex { x: 1, y: 2 }], 1.0);
    }

    #[test]
    fn compare_3x3_matrices() {
        let m1 = [[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]];
        let m2 = [[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]];
        assert_eq!(new_3x3(m1), new_3x3(m2));

        let m3 = [[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]];
        let m4 = [[2.0, 3.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]];
        assert_ne!(new_3x3(m3), new_3x3(m4));
    }

    #[test]
    fn create_2x2_matrix() {
        let mut test_matrix = [[-3.0, 5.0], [1.0, -2.0]];
        let test_m2x2 = new_2x2(test_matrix);
        assert_eq!(test_m2x2[&MatrixIndex { x: 0, y: 0 }], -3.0);
        assert_eq!(test_m2x2[&MatrixIndex { x: 1, y: 0 }], 5.0);
        assert_eq!(test_m2x2[&MatrixIndex { x: 0, y: 1 }], 1.0);
        assert_eq!(test_m2x2[&MatrixIndex { x: 1, y: 1 }], -2.0);
    }
    #[test]
    fn compare_2x2_matrices() {
        let m1 = [[1.0, 2.0], [5.5, 6.5]];
        let m2 = [[1.0, 2.0], [5.5, 6.5]];
        assert_eq!(new_2x2(m1), new_2x2(m2));

        let m3 = [[1.0, 2.0], [5.5, 6.5]];
        let m4 = [[2.0, 3.0], [5.5, 6.5]];
        assert_ne!(new_2x2(m3), new_2x2(m4));
    }
}
