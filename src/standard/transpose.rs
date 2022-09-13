//! Transposes matrices.

use crate::Matrix;

/// Transpose a matrix.
pub fn transpose(matrix: &Matrix) -> Matrix {
    let mut result = matrix.copy_vals();
    let cols = matrix.rows();
    let rows = matrix.cols();

    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            result[j*cols + i] = matrix[[i, j]];
        }
    }

    Matrix::new(rows, cols, result)
}