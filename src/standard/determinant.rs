//! Computes matrix determinants.

use crate::Matrix;

use super::get_minors;

/// Compute the determinant of a matrix, returning a floating-point value.
pub fn detf64(matrix: &Matrix) -> f64 {
    if matrix.rows() != matrix.cols() {
        // Square matrices only
        todo!()
    }

    let dim = matrix.rows();

    let det = if dim == 1 {
        matrix[[0, 0]]
    } else if dim == 2 {
        matrix[[0, 0]] * matrix[[1, 1]] - matrix[[1, 0]] * matrix[[0, 1]]
    } else {
        let mut d = 0.0f64;

        let mut sign = 1.0;

        // Laplace-expand the matrix down the first column
        for i in 0..dim {
            let minors = get_minors(&matrix, i, 0);

            d += matrix[[i, 0]] * detf64(&minors) * sign;

            if sign == 1.0 {
                sign = -1.0;
            } else {
                sign = 1.0;
            }
        }

        d
    };

    det
}


/// Compute the determinant of a matrix.
pub fn determinant(matrix: &Matrix) -> Matrix {
    Matrix::new(
        1,
        1,
        vec![detf64(matrix)]
    )
}