//! Generates Routh arrays for characteristic polynomials.

use crate::Matrix;
use crate::error::*;

use super::StdFunc;

#[derive(Clone)]
pub struct Routh;

impl Routh {
    pub fn evalpure(matrix: &Matrix) -> Matrix {
        if matrix.rows() != 1 {
            throw(RequiresUnitMatrix);
            return Matrix::new(0, 0, Vec::new());
        }

        todo!()
    }
}

impl StdFunc for Routh {
    fn eval(&self, args: Vec<Matrix>) -> Matrix {
        Matrix::new(0, 0, Vec::new())
    }
}