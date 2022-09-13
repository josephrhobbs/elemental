//! Generates identity matrices.

use crate::Matrix;

/// Generates an identity matrix of dimension `matrix[[0, 0]]`.
pub fn identity(matrix: &Matrix) -> Matrix {
    if matrix.rows() != 1 || matrix.cols() != 1 {
        todo!()
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