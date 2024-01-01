//! Exits the program.

use std::process::exit;

use crate::Matrix;

use super::StdFunc;

#[derive(Clone)]
pub struct Exit;

impl StdFunc for Exit {
    fn eval(&self, _: Vec<Matrix>) -> Matrix {
        exit(0);
    }
}