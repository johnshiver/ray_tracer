use crate::matrix::{new_4x4, M4x4, IDENTITY_MATRIX_4X4};

// moves a point
pub fn translation(x: f64, y: f64, z: f64) -> M4x4 {
    let mut base_matrix = IDENTITY_MATRIX_4X4;
    base_matrix.matrix[0][3] = x;
    base_matrix.matrix[1][3] = y;
    base_matrix.matrix[2][3] = z;
    new_4x4(base_matrix.matrix)
}

pub fn scaling(x: f64, y: f64, z: f64) -> M4x4 {
    let mut base_matrix = IDENTITY_MATRIX_4X4;
    base_matrix.matrix[0][0] = x;
    base_matrix.matrix[1][1] = y;
    base_matrix.matrix[2][2] = z;
    new_4x4(base_matrix.matrix)
}

pub fn rotation_x(radians: f64) -> M4x4 {
    let mut base_matrix = IDENTITY_MATRIX_4X4;
    base_matrix.matrix[1][1] = radians.cos();
    base_matrix.matrix[1][2] = -radians.sin();
    base_matrix.matrix[2][1] = radians.sin();
    base_matrix.matrix[2][2] = radians.cos();
    new_4x4(base_matrix.matrix)
}

pub fn rotation_y(radians: f64) -> M4x4 {
    let mut base_matrix = IDENTITY_MATRIX_4X4;
    base_matrix.matrix[0][0] = radians.cos();
    base_matrix.matrix[0][2] = radians.sin();
    base_matrix.matrix[2][0] = -radians.sin();
    base_matrix.matrix[2][2] = radians.cos();
    new_4x4(base_matrix.matrix)
}

fn rotation_z(radians: f64) -> M4x4 {
    let mut base_matrix = IDENTITY_MATRIX_4X4;
    base_matrix.matrix[0][0] = radians.cos();
    base_matrix.matrix[0][1] = -radians.sin();
    base_matrix.matrix[1][0] = radians.sin();
    base_matrix.matrix[1][1] = radians.cos();
    new_4x4(base_matrix.matrix)
}

fn shearing(xy: f64, xx: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> M4x4 {
    let mut base_matrix = IDENTITY_MATRIX_4X4;
    base_matrix.matrix[0][1] = xy;
    base_matrix.matrix[0][2] = xx;
    base_matrix.matrix[1][0] = yx;
    base_matrix.matrix[1][2] = yz;
    base_matrix.matrix[2][0] = zx;
    base_matrix.matrix[2][1] = zy;

    new_4x4(base_matrix.matrix)
}

#[cfg(test)]
mod tests {
    use crate::matrix::invert_4x4;
    use crate::matrix_transformations::{
        rotation_x, rotation_y, rotation_z, scaling, shearing, translation,
    };
    use crate::tuple::{new_point, new_vector};
    use std::borrow::Borrow;
    use std::f64::consts::PI;

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

    #[test]
    fn scale_matrix_applied_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = new_point(-4.0, 6.0, 8.0);
        let expected = new_point(-8.0, 18.0, 32.0);
        assert_eq!(transform * p, expected);
    }

    #[test]
    fn scale_matrix_applied_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = new_vector(-4.0, 6.0, 8.0);
        let expected = new_vector(-8.0, 18.0, 32.0);
        assert_eq!(transform * v, expected);
    }

    #[test]
    fn multi_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = invert_4x4(transform.borrow()).unwrap();
        let v = new_vector(-4.0, 6.0, 8.0);
        let expected = new_vector(-2.0, 2.0, 2.0);
        assert_eq!(inv * v, expected);
    }

    #[test]
    fn reflection_is_scaling_by_neg() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = new_point(2.0, 3.0, 4.0);
        let expected = new_point(-2.0, 3.0, 4.0);
        assert_eq!(transform * p, expected);
    }

    #[test]
    fn rotate_point_around_x_axis() {
        let p = new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        let exp1 = new_point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
        let exp2 = new_point(0.0, 0.0, 1.0);

        assert_eq!(half_quarter * p, exp1);
        assert_eq!(full_quarter * p, exp2);
    }

    #[test]
    fn inverse_x_rotation_rotates_opp_dir() {
        let p = new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = invert_4x4(half_quarter.borrow()).unwrap();
        let exp = new_point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);

        assert_eq!(inv * p, exp);
    }

    #[test]
    fn rotate_point_around_y_axis() {
        let p = new_point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        let exp1 = new_point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);
        let exp2 = new_point(1.0, 0.0, 0.0);

        assert_eq!(half_quarter * p, exp1);
        assert_eq!(full_quarter * p, exp2);
    }

    #[test]
    fn rotate_point_around_z_axis() {
        let p = new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        let exp1 = new_point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let exp2 = new_point(-1.0, 0.0, 0.0);

        assert_eq!(half_quarter * p, exp1);
        assert_eq!(full_quarter * p, exp2);
    }

    #[test]
    fn shearing_tx_moves_x_proportional_y() {
        let tx = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        let expected = new_point(5.0, 3.0, 4.0);
        assert_eq!(tx * p, expected)
    }

    #[test]
    fn shearing_tx_moves_x_proportional_z() {
        let tx = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        let expected = new_point(6.0, 3.0, 4.0);
        assert_eq!(tx * p, expected)
    }

    #[test]
    fn shearing_tx_moves_y_proportional_x() {
        let tx = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        let expected = new_point(2.0, 5.0, 4.0);
        assert_eq!(tx * p, expected)
    }

    #[test]
    fn shearing_tx_moves_y_proportional_z() {
        let tx = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        let expected = new_point(2.0, 7.0, 4.0);
        assert_eq!(tx * p, expected)
    }

    #[test]
    fn shearing_tx_moves_z_proportional_x() {
        let tx = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        let expected = new_point(2.0, 3.0, 6.0);
        assert_eq!(tx * p, expected)
    }

    #[test]
    fn shearing_tx_moves_z_proportional_y() {
        let tx = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = new_point(2.0, 3.0, 4.0);
        let expected = new_point(2.0, 3.0, 7.0);
        assert_eq!(tx * p, expected)
    }

    #[test]
    fn chaining_transformations_in_seq() {
        let p = new_point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, new_point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, new_point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, new_point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chaining_transformations_in_rev() {
        let p = new_point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let t = c * b * a;
        assert_eq!(t * p, new_point(15.0, 0.0, 7.0));
    }
}
