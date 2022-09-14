//! Defines dot products on vectors.

use crate::Matrix;
use crate::error::*;

use super::StdFunc;

#[derive(Clone)]
pub struct Dot;

impl Dot {
    /// Evaluates `Dot` while minimizing heap allocation.
    pub fn evalpure(vec1: &Matrix, vec2: &Matrix) -> Matrix {
        let a = vec1.vals();
        let b = vec2.vals();

        if a.len() != 3
            || b.len() != 3
            || vec1.rows() != vec2.rows()
            || vec1.cols() != vec2.cols()
        {
            throw(ImproperDimensions);
            return Matrix::new(0, 0, Vec::new());
        }

        let outputvec = vec![a[0]*b[0] + a[1]*b[1] + a[2]*b[2]];

        Matrix::new(1, 1, outputvec)
    }
}

impl StdFunc for Dot {
    fn eval(&self, args: Vec<Matrix>) -> Matrix {
        if args.len() != 2 {
            throw(WrongNumberOfArgs);
            return Matrix::new(0, 0, Vec::new());
        }

        Self::evalpure(&args[0], &args[1])
    }
}