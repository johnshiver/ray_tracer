use std::fmt;
use std::ops::{Index, Mul};

use thiserror::Error;

use crate::matrix::MatrixError::MatrixNotInvertible;
use crate::tuple::Tuple;
use crate::utils::equal_f64;

/// You know that you can multiply any number by 1 and get the original number.
/// The number 1 is called the multiplicative identity for that reason.
/// The identity matrix is like the number 1, but for matrices.
/// If you multiply any matrix or tuple by the identity matrix,
/// you get back the matrix or tuple you started with.
///
/// This may sound utterly pointless right now, but consider this:
/// if multiplying by the identity matrix just returns the original value,
/// it means you can use it as the default transformation for any object in your scene.
/// You don’t need any special cases to tell the difference between a
/// shape with a transformation and a shape without.
pub const IDENTITY_MATRIX_4X4: M4x4 = M4x4 {
    matrix: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

#[derive(Debug, Copy, Clone)]
struct MatrixIndex {
    x: usize,
    y: usize,
}

impl fmt::Display for MatrixIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl MatrixIndex {}

#[derive(Error, Debug)]
pub enum MatrixError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    #[error("matrix is not invertible")]
    MatrixNotInvertible,
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}

// ----------------------------- 4x4 ------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct M4x4 {
    pub matrix: [[f64; 4]; 4],
}

impl From<[[f64; 4]; 4]> for M4x4 {
    fn from(matrix: [[f64; 4]; 4]) -> Self {
        M4x4 { matrix }
    }
}

impl Index<MatrixIndex> for M4x4 {
    type Output = f64;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        match index {
            MatrixIndex { x: 0..=3, y: 0..=3 } => &self.matrix[index.y][index.x],
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
                if !(equal_f64(self[mi], other[mi])) {
                    return false;
                }
            }
        }
        true
    }
}

/// Matrix multiplication computes the dot product of every row-column combination in the two matrices
impl Mul<M4x4> for M4x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut new_matrix = [[0.0; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                new_matrix[y][x] = cal_index_matrix_multi(&self.matrix, &other.matrix, x, y);
            }
        }
        M4x4::from(new_matrix)
    }
}

/// Similar to multiplying by another matrix. Treats the tuple as a one column matrix
impl Mul<Tuple> for M4x4 {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        Tuple {
            x: cal_index_tuple_multi(&self.matrix, other, 0),
            y: cal_index_tuple_multi(&self.matrix, other, 1),
            z: cal_index_tuple_multi(&self.matrix, other, 2),
            w: cal_index_tuple_multi(&self.matrix, other, 3),
        }
    }
}

fn cal_index_matrix_multi(m1: &[[f64; 4]; 4], m2: &[[f64; 4]; 4], x: usize, y: usize) -> f64 {
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

fn cal_index_tuple_multi(m1: &[[f64; 4]; 4], t: Tuple, r: usize) -> f64 {
    // for y 1, x 0 of new matrix
    // line up row 1 for m1 and col 1 for m2
    let row = m1[r];
    t.x * row[0] + t.y * row[1] + t.z * row[2] + t.w * row[3]
}

/// Transposing a matrix turn its rows into columns and its columns into rows
///
/// Transposing the identity matrix will return the identity matrix
///
/// Useful when translating vectors between object space and world space
pub fn transpose(m: M4x4) -> M4x4 {
    let mut tx_m = [[0.0; 4]; 4];
    for y in 0..4 {
        for x in 0..4 {
            tx_m[x][y] = m.matrix[y][x];
        }
    }
    M4x4::from(tx_m)
}

/// Returns submatrix with given row and column removed
pub fn submatrix_4x4(matrix: &M4x4, row: usize, col: usize) -> M3x3 {
    let mut new_m = [[0.0; 3]; 3];
    let mut write_x = 0;
    let mut write_y = 0;
    for y in 0..4 {
        if y == row {
            continue;
        }
        for x in 0..4 {
            if x == col {
                continue;
            }
            let val = matrix.matrix[y][x];
            new_m[write_y][write_x] = val;
            write_x += 1;
        }
        write_x = 0;
        write_y += 1;
    }
    M3x3::from(new_m)
}

/// Minor is the determinant of given matrix's submatrix given row and column
///
pub fn minor_4x4(matrix: &M4x4, row: usize, col: usize) -> f64 {
    determinant_3x3(&submatrix_4x4(matrix, row, col))
}

pub fn cofactor_4x4(matrix: &M4x4, row: usize, col: usize) -> f64 {
    let cofactor = minor_4x4(matrix, row, col);
    if (row + col) % 2 == 0 {
        return cofactor;
    }
    -1.0 * cofactor
}

/// The determinant is a number that is derived from the elements of a matrix.
/// The name comes from the use of matrices to solve systems of equations,
/// where it’s used to determine whether or not the system has a solution.
/// If the determinant is zero, then the corresponding system of equations has no solution.
///
/// ChatGPT-4 explanation
/// The determinant serves as a tool to determine whether a set of vectors is linearly independent
/// by providing information about the "size" of the region they span.
/// If the determinant is non-zero, the vectors are linearly independent, whereas
/// a determinant of zero indicates that the vectors are linearly dependent.
pub fn determinant_4x4(matrix: &M4x4) -> f64 {
    let mut det = 0.0;
    for col in 0..4 {
        det += matrix.matrix[0][col] * cofactor_4x4(matrix, 0, col)
    }
    det
}

/// invertible_4x4
///
/// ChatGPT-4 explanation
/// An invertible matrix, also known as a non-singular matrix, is a matrix that has an inverse.
/// If a matrix A is invertible, there exists another matrix A^(-1) such that their product
/// results in the identity matrix:
///
/// A * A^(-1) = A^(-1) * A = I
///
/// A matrix is invertible if and only if its determinant is non-zero. This might seem like a simple
/// statement, but there is a deeper intuition behind it.
///
/// Recall that the determinant measures the "size" of the region spanned by the column vectors
/// of a matrix. In the 2D case, it represents the area of the parallelogram formed by the vectors;
/// in the 3D case, it represents the volume of the parallelepiped.
///
/// If the determinant is zero, it means that the region spanned by the
/// column vectors is degenerate, which implies that the vectors are linearly dependent.
/// In this case, the matrix cannot be inverted. This is because when you try to solve a
/// system of linear equations using a matrix with linearly dependent columns, there will be
/// either no unique solution (inconsistent system) or infinitely many solutions (dependent system).
///
/// On the other hand, if the determinant is non-zero, the region spanned by the column vectors
/// is non-degenerate, and the vectors are linearly independent. In this case,
/// the matrix can be inverted. This is because, for each output vector in the target space,
/// there exists a unique input vector that can be mapped to it using the matrix.
/// The inverse matrix can then be used to reverse this mapping,
/// taking output vectors back to their corresponding input vectors.
///
/// So, the intuition behind using the determinant to determine if a matrix is invertible is
/// that a non-zero determinant signifies that the column vectors of the matrix are linearly independent,
/// and the matrix defines a unique mapping from input vectors to output vectors. In this case,
/// the matrix can be inverted, and the inverse matrix can be used to reverse the mapping.
/// If the determinant is zero, the column vectors are linearly dependent, and the
/// matrix cannot be inverted, as it does not define a unique mapping.
pub fn invertible_4x4(matrix: &M4x4) -> bool {
    determinant_4x4(matrix) != 0.0
}

/// The idea of inverting a matrix is similar to the idea of inverting multiplication by dividing
/// If you multiply 5 * 4 you get 20. Divide 20 by 4 and you get 5.
/// Same idea for matrices. If you multiple matrix A by B you get C.
/// Multiply C by the inverse of B and you get A.
///
/// Inverting uses the cofactor expansion method
pub fn invert_4x4(matrix: &M4x4) -> Result<M4x4, MatrixError> {
    if !invertible_4x4(matrix) {
        return Err(MatrixNotInvertible);
    }
    let mut cofactors = [[0.0; 4]; 4];
    let det = determinant_4x4(matrix);
    for y in 0..4 {
        for x in 0..4 {
            let c = cofactor_4x4(matrix, y, x);
            // sneaky tricky to accomplish transpose operation
            cofactors[x][y] = c / det;
        }
    }
    Ok(M4x4::from(cofactors))
}
// ----------------------------- 3x3 ------------------------------------

#[derive(Debug)]
pub struct M3x3 {
    matrix: [[f64; 3]; 3],
}

impl Index<MatrixIndex> for M3x3 {
    type Output = f64;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        match index {
            MatrixIndex { x: 0..=2, y: 0..=2 } => &self.matrix[index.y][index.x],
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
                if !(equal_f64(self[mi], other[mi])) {
                    return false;
                }
            }
        }
        true
    }
}

impl From<[[f64; 3]; 3]> for M3x3 {
    fn from(matrix: [[f64; 3]; 3]) -> Self {
        M3x3 { matrix }
    }
}

pub fn submatrix_3x3(matrix: &M3x3, row: usize, col: usize) -> M2x2 {
    let mut new_m = [[0.0; 2]; 2];
    let mut write_x = 0;
    let mut write_y = 0;
    for y in 0..3 {
        if y == row {
            continue;
        }
        for x in 0..3 {
            if x == col {
                continue;
            }
            let val = matrix.matrix[y][x];
            new_m[write_y][write_x] = val;
            write_x += 1;
        }
        write_x = 0;
        write_y += 1;
    }
    M2x2::from(new_m)
}

pub fn minor_3x3(matrix: &M3x3, row: usize, col: usize) -> f64 {
    determinant_2x2(&submatrix_3x3(matrix, row, col))
}

pub fn cofactor_3x3(matrix: &M3x3, row: usize, col: usize) -> f64 {
    let cofactor = minor_3x3(matrix, row, col);
    if (row + col) % 2 == 0 {
        return cofactor;
    }
    -1.0 * cofactor
}

pub fn determinant_3x3(matrix: &M3x3) -> f64 {
    let mut det = 0.0;
    for col in 0..3 {
        det += matrix.matrix[0][col] * cofactor_3x3(matrix, 0, col)
    }
    det
}

// ----------------------------- 2x2 ------------------------------------
#[derive(Debug)]
pub struct M2x2 {
    matrix: [[f64; 2]; 2],
}

impl From<[[f64; 2]; 2]> for M2x2 {
    fn from(matrix: [[f64; 2]; 2]) -> Self {
        M2x2 { matrix }
    }
}

impl Index<MatrixIndex> for M2x2 {
    type Output = f64;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        match index {
            MatrixIndex { x: 0..=1, y: 0..=1 } => &self.matrix[index.y][index.x],
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
                if !(equal_f64(self[mi], other[mi])) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn determinant_2x2(m: &M2x2) -> f64 {
    (m.matrix[0][0] * m.matrix[1][1]) - (m.matrix[0][1] * m.matrix[1][0])
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::matrix::{
        cofactor_3x3, cofactor_4x4, determinant_2x2, determinant_3x3, determinant_4x4, invert_4x4,
        invertible_4x4, minor_3x3, submatrix_3x3, submatrix_4x4, transpose, M2x2, M3x3, M4x4,
        MatrixIndex, IDENTITY_MATRIX_4X4,
    };
    use crate::tuple::{Point, Tuple};

    #[test]
    fn create_4x4_matrix() {
        let test_matrix = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];
        let test_m4x4 = M4x4::from(test_matrix);
        assert_eq!(test_m4x4[MatrixIndex { x: 0, y: 0 }], 1.0);
        assert_eq!(test_m4x4[MatrixIndex { x: 3, y: 0 }], 4.0);
        assert_eq!(test_m4x4[MatrixIndex { x: 0, y: 1 }], 5.5);
        assert_eq!(test_m4x4[MatrixIndex { x: 2, y: 1 }], 7.5);
        assert_eq!(test_m4x4[MatrixIndex { x: 2, y: 2 }], 11.0);
        assert_eq!(test_m4x4[MatrixIndex { x: 0, y: 3 }], 13.5);
        assert_eq!(test_m4x4[MatrixIndex { x: 2, y: 3 }], 15.5);
    }

    #[test]
    fn compare_4x4_matrices() {
        let m1 = M4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        let m2 = M4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m1, m2);

        let m3 = M4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        let m4 = M4x4::from([
            [2.0, 3.0, 3.0, 5.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_ne!(m3, m4);
    }

    #[test]
    fn multiply_4x4_matrices() {
        let m1 = M4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = M4x4::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let expected = M4x4::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(m1 * m2, expected);
    }

    #[test]
    fn multiply_4x4_matrix_tuple() {
        let m1 = M4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let point = Point::new_point(1.0, 2.0, 3.0);
        let expected = Tuple {
            x: 18.0,
            y: 24.0,
            z: 33.0,
            w: 1.0,
        };

        assert_eq!(m1 * point, expected);
    }

    #[test]
    fn transpose_4x4_matrix() {
        let m1 = M4x4::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let expected = M4x4::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(transpose(m1), expected);
        assert_eq!(transpose(IDENTITY_MATRIX_4X4), IDENTITY_MATRIX_4X4);
    }

    #[test]
    fn create_3x3_matrix() {
        let test_matrix = [[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]];
        let test_m3x3 = M3x3::from(test_matrix);
        assert_eq!(test_m3x3[MatrixIndex { x: 0, y: 0 }], -3.0);
        assert_eq!(test_m3x3[MatrixIndex { x: 1, y: 1 }], -2.0);
        assert_eq!(test_m3x3[MatrixIndex { x: 2, y: 2 }], 1.0);
        assert_eq!(test_m3x3[MatrixIndex { x: 1, y: 2 }], 1.0);
    }

    #[test]
    fn compare_3x3_matrices() {
        let m1 = M3x3::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        let m2 = M3x3::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        assert_eq!(m1, m2);

        let m3 = M3x3::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        let m4 = M3x3::from([[2.0, 3.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        assert_ne!(m3, m4);
    }

    #[test]
    fn create_2x2_matrix() {
        let m = M2x2::from([[-3.0, 5.0], [1.0, -2.0]]);
        assert_eq!(m[MatrixIndex { x: 0, y: 0 }], -3.0);
        assert_eq!(m[MatrixIndex { x: 1, y: 0 }], 5.0);
        assert_eq!(m[MatrixIndex { x: 0, y: 1 }], 1.0);
        assert_eq!(m[MatrixIndex { x: 1, y: 1 }], -2.0);
    }

    #[test]
    fn compare_2x2_matrices() {
        let m1 = M2x2::from([[1.0, 2.0], [5.5, 6.5]]);
        let m2 = M2x2::from([[1.0, 2.0], [5.5, 6.5]]);
        assert_eq!(m1, m2);

        let m3 = M2x2::from([[1.0, 2.0], [5.5, 6.5]]);
        let m4 = M2x2::from([[2.0, 3.0], [5.5, 6.5]]);
        assert_ne!(m3, m4);
    }

    #[test]
    fn determinant_2x2_matrices() {
        let m1 = M2x2::from([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(determinant_2x2(&m1), 17.0);
    }

    #[test]
    fn identity_matrix_multi_matrix() {
        let m1 = M4x4::from([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        assert_eq!(m1 * IDENTITY_MATRIX_4X4, m1)
    }

    #[test]
    fn identity_matrix_multi_tuple() {
        let expected = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        assert_eq!(IDENTITY_MATRIX_4X4 * expected, expected)
    }

    #[test]
    fn submatrix_3x3_2x2() {
        let test_m3 = M3x3::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let expected = M2x2::from([[-3.0, 2.0], [0.0, 6.0]]);
        assert_eq!(submatrix_3x3(&test_m3, 0, 2), expected);
    }

    #[test]
    fn submatrix_4x4_3x3() {
        let test_m3 = M4x4::from([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let expected = M3x3::from([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        assert_eq!(submatrix_4x4(&test_m3, 2, 1), expected);
    }

    #[test]
    fn minor_3x3_test() {
        let a = M3x3::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(minor_3x3(&a, 1, 0), 25.0);
    }

    #[test]
    fn cofactor_3x3_test() {
        let a = M3x3::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(cofactor_3x3(&a, 0, 0), -12.0);
        assert_eq!(cofactor_3x3(&a, 1, 0), -25.0);
    }

    #[test]
    fn determinant_3x3_test() {
        let a = M3x3::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(determinant_3x3(&a), -196.0);
    }

    #[test]
    fn determinant_4x4_test() {
        let a = M4x4::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(determinant_4x4(&a), -4071.0);
    }

    #[test]
    fn matrix_invertibility() {
        let a = M4x4::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(determinant_4x4(&a), -2120.0);
        assert!(invertible_4x4(&a));

        let a = M4x4::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(determinant_4x4(&a), 0.0);
        assert!(!invertible_4x4(&a))
    }

    #[test]
    fn matrix_inverse() {
        let a = M4x4::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        assert_eq!(determinant_4x4(&a), 532.0);
        assert_eq!(cofactor_4x4(&a, 2, 3), -160.0);
        assert_eq!(cofactor_4x4(&a, 3, 2), 105.0);

        let b = invert_4x4(a.borrow()).unwrap();
        let expected = M4x4::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert_eq!(b, expected);

        let c = M4x4::from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let d = invert_4x4(&c).unwrap();
        let e2 = M4x4::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_eq!(d, e2);

        let e = M4x4::from([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let f = invert_4x4(&e).unwrap();
        let e3 = M4x4::from([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06663, -0.26667, 0.333333333333333],
        ]);
        assert_eq!(f, e3);
    }

    #[test]
    fn matrix_product_by_its_inverse() {
        let a = M4x4::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let b = M4x4::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a * b;
        assert_eq!(a, c * invert_4x4(&b).unwrap());
    }
}
