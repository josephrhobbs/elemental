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

/// Pure implementation of `determinant` for recursive calls.
fn det_matrix(values: Vec<Expression>) -> Expression {
    if values.len() == 4 {
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
        let _determinant = 0;

        for _i in 0..((values.len() as f64).sqrt() as usize + 1) {
            
        }

        todo!()
    }
}