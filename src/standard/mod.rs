//! The standard library for Elemental.
//! 
//! This library defines all built-in functions for the Elemental language.
//! It exports a `HashMap` to the main interpreter, allowing the interpreter
//! to connect function names to function definitions.

use std::collections::HashMap;

use crate::Expression;


/// Get a function pointer based on that function's name.
pub fn get_std_function(name: String) -> fn(Vec<Expression>) -> Expression {
    let mut hashmap: HashMap<String, fn(Vec<Expression>) -> Expression> = HashMap::new();

    // Declarative standard library begins here
    hashmap.insert("t".to_string(), transpose);
    hashmap.insert("det".to_string(), determinant);

    match hashmap.get(&name) {
        Some(f) => *f,
        None => todo!(),
    }
}


/// Transpose a matrix.
fn transpose(args: Vec<Expression>) -> Expression {
    if args.len() != 1 {
        todo!()
    }

    if let Expression::Matrix {
        rows: r,
        cols: c,
        values: v,
    } = &args[0] {
        let mut return_values = vec![Expression::Nil; r*c];

        for i in 0..*r {
            for j in 0..*c {
                return_values[j*r + i] = v[i*c + j].to_owned();
            }
        }

        Expression::Matrix {
            rows: *c,
            cols: *r,
            values: return_values,
        }
    } else {
        todo!()
    }
}


/// Compute the determinant of a matrix.
fn determinant(args: Vec<Expression>) -> Expression {
    if args.len() != 1 {
        todo!()
    }

    if let Expression::Matrix {
        rows: r,
        cols: c,
        values: v,
    } = &args[0] {
        if *r != *c {
            todo!()
        }
        det_matrix(v.to_owned())
    } else {
        todo!()
    }
}

/// Gets the matrix of minors (excluding row `row` and column `1`) given the values of a matrix.
fn get_minors(values: Vec<Expression>, cols: usize, row: usize) -> Vec<Expression> {
    let mut output = Vec::new();

    for i in 0..values.len() {
        if i%cols != 0 && i/cols != row {
            output.push(values[i].to_owned());
        }
    }

    output
}

/// Pure implementation of `determinant` for recursive calls.
fn det_matrix(values: Vec<Expression>) -> Expression {
    let dim = (values.len() as f64).sqrt() as usize;
    if dim == 1 {
        values[0].to_owned()
    } else if dim == 2 {
        Expression::BinOp {
            left: Box::new(Expression::BinOp {
                left: Box::new(values[0].to_owned()),
                op: "*".to_string(),
                right: Box::new(values[3].to_owned()),
            }),
            right: Box::new(Expression::BinOp {
                left: Box::new(values[1].to_owned()),
                op: "*".to_string(),
                right: Box::new(values[2].to_owned()),
            }),
            op: "-".to_string(),
        }
    } else {
        let mut determinant = Expression::Int (0);

        let mut operator = String::from("+");

        for i in 0..dim {
            determinant = Expression::BinOp {
                left: Box::new(determinant.to_owned()),
                op: operator.to_owned(),
                right: Box::new(Expression::BinOp {
                    left: Box::new(values[i*dim].to_owned()),
                    op: "*".to_string(),
                    right: Box::new(det_matrix(get_minors(values.to_owned(), dim, i))),
                }),
            };

            operator = if operator == String::from("+") {
                String::from("-")
            } else {
                String::from("+")
            };
        }

        determinant
    }
}