//! Computes matrices of minors.

use crate::Matrix;

/// Gets the matrix of minors (excluding row `row` and column `col`) given the values of a matrix.
pub fn get_minors(matrix: &Matrix, row: usize, col: usize) -> Matrix {
    let mut values = Vec::new();

    let rows = matrix.rows();
    let cols = matrix.cols();

    for i in 0..rows {
        for j in 0..cols {
            if i != row && j != col {
                values.push(matrix[[i, j]])
            }
        }
    }

    Matrix::new(rows - 1, cols - 1, values)
}