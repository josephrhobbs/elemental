//! Inverts matrices.

use crate::Matrix;

use super::{
    get_minors,
    transpose,
    detf64,
};

/// Computes the inverse of the given matrix.
pub fn invert(matrix: &Matrix) -> Matrix {
    let mut output = matrix.clone();
    let rows = matrix.rows();
    let cols = matrix.cols();

    // Compute the determinant of the input matrix
    let original_det = detf64(matrix);

    // Step 1: For each cell, compute the determinant of the matrix of minors
    // determined with respect to that cell
    for i in 0..rows {
        for j in 0..cols {
            let matrix_of_minors = get_minors(matrix, i, j);
            let det = detf64(&matrix_of_minors);
            output[[i, j]] = det;
        }
    }

    // Step 2: For each cell, if `i + j` is odd, multiply the value by -1
    // Step 4: Divide each value by the determinant of the original matrix
    for i in 0..rows {
        for j in 0..cols {
            if (i + j)%2 != 0 {
                output[[i, j]] = output[[i, j]] * -1.0f64;
            }
            output[[i, j]] = output[[i, j]] / original_det;
        }
    }

    // Step 3: Transpose the matrix
    output = transpose(&output);

    // Did Step 4 above

    // We're done!
    output
}