//! Computes matrix determinants.

use crate::Matrix;

use super::{
    StdFunc,
    GetMinors,   
};

#[derive(Clone)]
pub struct Determinant;

impl Determinant {
    pub fn evalpure(matrix: &Matrix) -> f64 {
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
                let minors = GetMinors::evalpure(&matrix, i, 0);
    
                d += matrix[[i, 0]] * Determinant::evalpure(&minors) * sign;
    
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
}

impl StdFunc for Determinant {
    fn eval(&self, args: Vec<Matrix>) -> Matrix {
        if args.len() != 1 {
            todo!();
        }

        Matrix::new(1, 1, vec![Self::evalpure(&args[0])])
    }
}