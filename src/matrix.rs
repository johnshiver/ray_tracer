use std::borrow::Borrow;
use std::ops::Index;

struct MatrixIndex {
    x: usize,
    y: usize,
}

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

impl M4x4 {}

pub fn new_4x4(matrix: [[f64; 4]; 4]) -> M4x4 {
    M4x4 { matrix }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{new_4x4, MatrixIndex};

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
}
