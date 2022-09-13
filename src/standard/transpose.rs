//! Transposes matrices.

use crate::Matrix;
use crate::error::*;

use super::StdFunc;

/// Transposes a matrix.
#[derive(Clone)]
pub struct Transpose;

impl Transpose {
    /// Evaluates `Transpose` while minimizing heap allocation.
    pub fn evalpure(matrix: &Matrix) -> Matrix {
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
}

impl StdFunc for Transpose {
    fn eval(&self, args: Vec<Matrix>) -> Matrix {
        if args.len() != 1 {
            throw(WrongNumberOfArgs);
            return Matrix::new(0, 0, Vec::new());
        }

        Self::evalpure(&args[0])
    }
}