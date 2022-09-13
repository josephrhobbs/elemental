//! Inverts matrices.

use crate::Matrix;
use crate::error::*;

use super::{
    StdFunc,
    GetMinors,
    Transpose,
    Determinant,
};

#[derive(Clone)]
pub struct Invert;

impl Invert {
    /// Evaluates `Invert` while minimizing heap allocation.
    pub fn evalpure(matrix: &Matrix) -> Matrix {
        let mut output = matrix.clone();
        let rows = matrix.rows();
        let cols = matrix.cols();

        // Compute the determinant of the input matrix
        let original_det = Determinant::evalpure(matrix);

        // Step 1: For each cell, compute the determinant of the matrix of minors
        // determined with respect to that cell
        for i in 0..rows {
            for j in 0..cols {
                let matrix_of_minors = GetMinors::evalpure(matrix, i, j);
                let det = Determinant::evalpure(&matrix_of_minors);
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
        output = Transpose::evalpure(&output);

        // Did Step 4 above

        // We're done!
        output
    }
}

impl StdFunc for Invert {
    fn eval(&self, args: Vec<Matrix>) -> Matrix {
        if args.len() != 1 {
            throw(WrongNumberOfArgs);
            return Matrix::new(0, 0, Vec::new());
        }

        Self::evalpure(&args[0])
    }
}