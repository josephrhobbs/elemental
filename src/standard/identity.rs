//! Generates identity matrices.

use crate::Matrix;
use crate::error::*;

use super::StdFunc;

#[derive(Clone)]
pub struct Identity;

impl Identity {
    pub fn evalpure(matrix: &Matrix) -> Matrix {
        if matrix.rows() != 1 || matrix.cols() != 1 {
            throw(RequiresUnitMatrix);
            return Matrix::new(0, 0, Vec::new());
        }
    
        let dim = matrix[[0, 0]] as usize;
    
        let mut output = Matrix::new(dim, dim, vec![0.0; dim*dim]);
    
        for i in 0..dim {
            for j in 0..dim {
                if i == j {
                    output[[i, j]] = 1.0;
                }
            }
        }
    
        output
    }
}

impl StdFunc for Identity {
    fn eval(&self, args: Vec<Matrix>) -> Matrix {
        if args.len() != 1 {
            throw(WrongNumberOfArgs);
            return Matrix::new(0, 0, Vec::new());
        }

        Self::evalpure(&args[0])
    }
}