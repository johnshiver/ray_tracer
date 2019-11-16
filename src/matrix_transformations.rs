use crate::matrix::{M4x4, new_4x4, IDENTITY_MATRIX_4X4};

// moves a point
fn translation(x: f64, y: f64, z: f64) -> M4x4 {
    let mut base_matrix = IDENTITY_MATRIX_4X4.clone();
    base_matrix.matrix[0][3] = x;
    base_matrix.matrix[1][3] = y;
    base_matrix.matrix[2][3] = z;
    new_4x4(base_matrix.matrix)
}


#[cfg(test)]
mod tests {
    use crate::tuple::{new_point, new_vector};
    use crate::matrix_transformations::translation;
    use crate::matrix::invert_4x4;
    use std::borrow::Borrow;

    #[test]
    fn translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = new_point(-3.0, 4.0, 5.0);
        let expected = new_point(2.0, 1.0, 7.0);
        assert_eq!(transform * p, expected)
    }

    #[test]
    fn translation_matrix_inversion() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = invert_4x4(transform.borrow()).unwrap();
        let p = new_point(-3.0, 4.0, 5.0);
        let expected = new_point(-8.0, 7.0, 3.0);
        assert_eq!(inv * p, expected)
    }

    #[test]
    fn multi_translation_matrix_vector() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = new_vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

}