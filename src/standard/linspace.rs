//! Generates linear spaces (linspaces).

use crate::Matrix;
use crate::error::*;

use super::StdFunc;

#[derive(Clone)]
pub struct Linspace;

impl Linspace {
    /// Evaluates `Linspace` while minimizing heap allocation.
    pub fn evalpure(start: f64, end: f64, count: f64) -> Matrix {
        // Arbitrary step computed from count given
        let mut current = start;
        let count = count as i64;
        let step = (end - current)/(count as f64);

        let mut output = Vec::new();

        for _ in 0..count {
            output.push(current);
            current += step;
        }

        Matrix::new(1, output.len(), output)
    }
}

impl StdFunc for Linspace {
    fn eval(&self, args: Vec<Matrix>) -> Matrix {
        if args.len() == 2 {
            let count = (args[1][[0, 0]] - args[0][[0, 0]]).round();
            Self::evalpure(args[0][[0, 0]], args[1][[0, 0]], count)
        } else if args.len() == 3 {
            Self::evalpure(args[0][[0, 0]], args[1][[0, 0]], args[2][[0, 0]])
        } else {
            throw(WrongNumberOfArgs);
            return Matrix::empty();
        }
    }
}