//! The standard library for Elemental.
//! 
//! This library defines all built-in functions for the Elemental language.
//! It exports a `HashMap` to the main interpreter, allowing the interpreter
//! to connect function names to function definitions.

use std::collections::HashMap;

use crate::Matrix;


/// Get a function pointer based on that function's name.
pub fn get_std_function(name: String) -> fn(Vec<Matrix>) -> Matrix {
    let mut hashmap: HashMap<String, fn(Vec<Matrix>) -> Matrix> = HashMap::new();

    // Declarative standard library begins here
    hashmap.insert("t".to_string(), transpose);
    hashmap.insert("det".to_string(), determinant);
    hashmap.insert("I".to_string(), identity);
    hashmap.insert("inv".to_string(), invert);

    match hashmap.get(&name) {
        Some(f) => *f,
        None => todo!(),
    }
}


/// Transpose a matrix.
fn transpose(args: Vec<Matrix>) -> Matrix {
    if args.len() != 1 {
        // Too many arguments passed!
        todo!()
    }

    let matrix = args[0].clone();
    let mut result = matrix.copy_vals();
    let cols = matrix.rows();
    let rows = matrix.cols();

    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            result[j*cols + i] = matrix[[i, j]];
        }
    }

    Matrix::new(rows, cols, result)
}


/// Compute the determinant of a matrix.
fn determinant(args: Vec<Matrix>) -> Matrix {
    if args.len() != 1 {
        // Too many arguments passed!
        todo!()
    }

    let matrix = args[0].clone();
    
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
            let minors = get_minors(&matrix, i, 0);

            d += matrix[[i, 0]] * determinant(vec![minors])[[0, 0]] * sign;

            if sign == 1.0 {
                sign = -1.0;
            } else {
                sign = 1.0;
            }
        }

        d
    };

    Matrix::new(1, 1, vec![det])
}

/// Gets the matrix of minors (excluding row `row` and column `col`) given the values of a matrix.
fn get_minors(matrix: &Matrix, row: usize, col: usize) -> Matrix {
    let mut values = Vec::new();

    let rows = matrix.rows();
    let cols = matrix.cols();

    for i in 0..rows {
        for j in 0..cols {
            if i != row && j != col {
                values.push(matrix[[i, j]])
            }
        }
    }

    Matrix::new(rows - 1, cols - 1, values)
}


/// Generates an identity matrix of dimension `matrix[[0, 0]]`.
fn identity(args: Vec<Matrix>) -> Matrix {
    if args.len() != 1 {
        todo!()
    }

    let matrix = args[0].clone();

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


/// Computes the inverse of the given matrix.
fn invert(args: Vec<Matrix>) -> Matrix {
    if args.len() != 1 {
        todo!()
    }

    let mut matrix = args[0].clone();
    let rows = matrix.rows();
    let cols = matrix.cols();

    // Keep an original copy
    let original = matrix.clone();
    let original_det = determinant(vec![original.to_owned()])[[0, 0]];

    // Step 1: For each cell, compute the determinant of the matrix of minors
    // determined with respect to that cell
    for i in 0..rows {
        for j in 0..cols {
            let matrix_of_minors = get_minors(&original, i, j);
            let det = determinant(vec![matrix_of_minors])[[0, 0]];
            matrix[[i, j]] = det;
        }
    }

    // Step 2: For each cell, if `i + j` is odd, multiply the value by -1
    // Step 4: Divide each value by the determinant of the original matrix
    for i in 0..rows {
        for j in 0..cols {
            if (i + j)%2 != 0 {
                matrix[[i, j]] = matrix[[i, j]] * -1.0f64;
            }
            matrix[[i, j]] = matrix[[i, j]] / original_det;
        }
    }

    // Step 3: Transpose the matrix
    matrix = transpose(vec![matrix]);

    // Did Step 4 above

    // We're done!
    matrix.to_owned()
}