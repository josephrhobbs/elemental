//! Computes sines.

use crate::Matrix;
use crate::error::*;

use super::StdFunc;

#[derive(Clone)]
pub struct Sin;

impl Sin {
    /// Evaluates `Invert` while minimizing heap allocation.
    pub fn evalpure(matrix: &Matrix) -> Matrix {
        let mut output = Vec::new();
        let rows = matrix.rows();
        let cols = matrix.cols();

        for v in matrix.vals() {
            output.push(v.sin());
        }

        Matrix::new(
            rows,
            cols,
            output,
        )
    }
}

impl StdFunc for Sin {
    fn eval(&self, args: Vec<Matrix>) -> Matrix {
        if args.len() != 1 {
            throw(WrongNumberOfArgs);
            return Matrix::new(0, 0, Vec::new());
        }

        Self::evalpure(&args[0])
    }
}