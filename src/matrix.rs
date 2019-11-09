use std::borrow::Borrow;
use std::ops::Index;

struct MatrixIndex {
    x: usize,
    y: usize,
}

// ----------------------------- 4x4 ------------------------------------
pub struct M4x4 {
    matrix: [[f64; 4]; 4],
}

impl Index<MatrixIndex> for M4x4 {
    type Output = f64;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        match index {
            MatrixIndex { x: 0..=3, y: 0..=3 } => self.matrix[index.y][index.x].borrow(),
            _ => &-99.0,
        }
    }
}

pub fn new_4x4(matrix: [[f64; 4]; 4]) -> M4x4 {
    M4x4 { matrix }
}

// ----------------------------- 3x3 ------------------------------------

pub struct M3x3 {
    matrix: [[f64; 3]; 3],
}

impl Index<MatrixIndex> for M3x3 {
    type Output = f64;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        match index {
            MatrixIndex { x: 0..=2, y: 0..=2 } => self.matrix[index.y][index.x].borrow(),
            _ => &-99.0,
        }
    }
}

pub fn new_3x3(matrix: [[f64; 3]; 3]) -> M3x3 {
    M3x3 { matrix }
}

// ----------------------------- 2x2 ------------------------------------
pub struct M2x2 {
    matrix: [[f64; 2]; 2],
}

impl Index<MatrixIndex> for M2x2 {
    type Output = f64;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        match index {
            MatrixIndex { x: 0..=1, y: 0..=1 } => self.matrix[index.y][index.x].borrow(),
            _ => &-99.0,
        }
    }
}

pub fn new_2x2(matrix: [[f64; 2]; 2]) -> M2x2 {
    M2x2 { matrix }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{new_2x2, new_3x3, new_4x4, MatrixIndex};

    #[test]
    fn create_4x4_matrix() {
        let mut test_matrix = [[0.0; 4]; 4];
        test_matrix[0] = [1.0, 2.0, 3.0, 4.0];
        test_matrix[1] = [5.5, 6.5, 7.5, 8.5];
        test_matrix[2] = [9.0, 10.0, 11.0, 12.0];
        test_matrix[3] = [13.5, 14.5, 15.5, 16.5];
        let test_m4x4 = new_4x4(test_matrix);
        assert_eq!(test_m4x4[MatrixIndex { x: 0, y: 0 }], 1.0);
        assert_eq!(test_m4x4[MatrixIndex { x: 3, y: 0 }], 4.0);
        assert_eq!(test_m4x4[MatrixIndex { x: 0, y: 1 }], 5.5);
        assert_eq!(test_m4x4[MatrixIndex { x: 2, y: 1 }], 7.5);
        assert_eq!(test_m4x4[MatrixIndex { x: 2, y: 2 }], 11.0);
        assert_eq!(test_m4x4[MatrixIndex { x: 0, y: 3 }], 13.5);
        assert_eq!(test_m4x4[MatrixIndex { x: 2, y: 3 }], 15.5);
    }

    #[test]
    fn create_3x3_matrix() {
        let mut test_matrix = [[0.0; 3]; 3];
        test_matrix[0] = [-3.0, 5.0, 0.0];
        test_matrix[1] = [1.0, -2.0, -7.0];
        test_matrix[2] = [0.0, 1.0, 1.0];
        let test_m3x3 = new_3x3(test_matrix);
        assert_eq!(test_m3x3[MatrixIndex { x: 0, y: 0 }], -3.0);
        assert_eq!(test_m3x3[MatrixIndex { x: 1, y: 1 }], -2.0);
        assert_eq!(test_m3x3[MatrixIndex { x: 2, y: 2 }], 1.0);
        assert_eq!(test_m3x3[MatrixIndex { x: 1, y: 2 }], 1.0);
    }

    #[test]
    fn create_2x2_matrix() {
        let mut test_matrix = [[0.0; 2]; 2];
        test_matrix[0] = [-3.0, 5.0];
        test_matrix[1] = [1.0, -2.0];
        let test_m2x2 = new_2x2(test_matrix);
        assert_eq!(test_m2x2[MatrixIndex { x: 0, y: 0 }], -3.0);
        assert_eq!(test_m2x2[MatrixIndex { x: 1, y: 0 }], 5.0);
        assert_eq!(test_m2x2[MatrixIndex { x: 0, y: 1 }], 1.0);
        assert_eq!(test_m2x2[MatrixIndex { x: 1, y: 1 }], -2.0);
    }
}
