//! Provides an abstraction over matrix behaviors.

use std::ops::{
    Add,
    Sub,
    Mul,
    Index,
    IndexMut,
};

use crate::error::*;

/// Abstracts over matrices.
#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    vals: Vec<f64>,
}

impl Matrix {
    /// Constructs a new matrix.
    pub fn new(rows: usize, cols: usize, vals: Vec<f64>) -> Self {
        Self {
            rows,
            cols,
            vals,
        }
    }

    /// Multiplies the given matrix by the given scalar, returning a new matrix.
    pub fn scalar_multiply(&self, scalar: f64) -> Self {
        let vals = self.vals.iter().map(|x| scalar*x).collect::<Vec<f64>>();

        Self {
            vals,
            ..*self
        }
    }

    /// Gets the number of rows of the matrix.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Gets the number of columns of the matrix.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Copies the values of the matrix.
    pub fn copy_vals(&self) -> Vec<f64> {
        self.vals.to_owned()
    }

    /// Gets the values of the matrix.
    pub fn vals(&self) -> &Vec<f64> {
        &self.vals
    }
    
    /// Gets the values of the matrix, with mutable permission.
    pub fn vals_mut(&mut self) -> &mut Vec<f64> {
        &mut self.vals
    }
}

/// Defines matrix addition.
impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            throw(ImproperDimensions);
            Self::new(0, 0, Vec::new());
        }

        let mut output_vals = Vec::new();

        for (i, j) in self.vals().iter().zip(other.vals().iter()) {
            output_vals.push(i + j);
        }

        Self {
            vals: output_vals,
            ..self
        }
    }
}

/// Defines matrix subtraction.
impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            throw(ImproperDimensions);
            Self::new(0, 0, Vec::new());
        }

        let mut output_vals = Vec::new();

        for (i, j) in self.vals().iter().zip(other.vals().iter()) {
            output_vals.push(i - j);
        }

        Self {
            vals: output_vals,
            ..self
        }
    }
}

/// Defines matrix multiplication.
impl Mul for Matrix {
    type Output = Self;

    #[allow(unused_variables)]
    fn mul(self, other: Self) -> Self {
        Self::new(0, 0, Vec::new())
    }
}

/// Defines matrix indexing.
impl Index<[usize; 2]> for Matrix {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let i = index[0];
        let j = index[1];
        &self.vals()[i*self.cols() + j]
    }
}

/// Defines matrix indexing with mutable permission.
impl IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let i = index[0];
        let j = index[1];
        let cols = self.cols();
        &mut self.vals_mut()[i*cols + j]
    }
}