//! Generates rotation matrices.

use crate::Matrix;
use crate::error::*;

use super::StdFunc;

#[derive(Clone)]
pub struct Rotation2d;

impl Rotation2d {
    pub fn evalpure(matrix: &Matrix) -> Matrix {
        if matrix.rows() != 1 || matrix.cols() != 1 {
            throw(RequiresUnitMatrix);
            return Matrix::new(0, 0, Vec::new());
        }
    
        let angle = matrix[[0, 0]];
    
        Matrix::new(
            2,
            2,
            vec![
                angle.cos(),
                -angle.sin(),
                angle.sin(),
                angle.cos(),
            ]
        )
    }
}

impl StdFunc for Rotation2d {
    fn eval(&self, args: Vec<Matrix>) -> Matrix {
        if args.len() != 1 {
            throw(WrongNumberOfArgs);
            return Matrix::new(0, 0, Vec::new());
        }

        Self::evalpure(&args[0])
    }
}