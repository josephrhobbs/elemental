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
    hashmap.insert("i".to_string(), identity);

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
    let mut result = matrix.clone();

    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            result[[i, j]] = matrix[[j, i]]
        }
    }

    result
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

    let det = if dim == 2 {
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