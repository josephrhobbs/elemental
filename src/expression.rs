//! Abstracts over Elemental expressions.

use std::{
    fmt::{
        Display,
        Result,
        Formatter,
    },
    collections::HashMap,
};

use crate::{
    standard::get_std_function,
    Matrix,
};

use crate::error::*;

/// Defines the expression types that are available in Elemental.
#[derive(Clone, Debug)]
pub enum Expression {
    Assignment {
        identifier: String,
        value: Box<Expression>,
    },
    Identifier (String),
    Int (i64),
    Float (f64),
    Matrix {
        rows: usize,
        cols: usize,
        values: Vec<Expression>,
    },
    BinOp {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
    Call {
        name: String,
        args: Vec<Expression>,
    },
    Nil,
}

/// Implementing `std::fmt::Display` allows us to print expressions
/// using the default formatter.
impl Display for Expression {
    /// Display each `Expression`.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Assignment {
                identifier: _,
                value: v,
            } => {
                write!(f, "{}", v)
            },
            Expression::Identifier (s) => {
                write!(f, "{}", s)
            },
            Expression::Int (i) => {
                write!(f, "{}", i)
            },
            Expression::Float (float) => {
                write!(f, "{:.8}", float)
            },
            Expression::Matrix {
                rows: r,
                cols: c,
                values: v,
            } => {
                let mut result = String::new();
                for i in 0..*r {
                    result.push('[');
                    for j in 0..*c {
                        let index = i*c + j;
                        result.push_str(
                            &format!(
                                "{:^10}",
                                format!("{}", v[index])
                            )
                        );

                        // Write a tab if we're not at the end yet
                        if j != c - 1 {
                            result.push(' ');
                        }
                    }
                    result.push(']');
                    result.push('\n');
                }
                write!(f, "{}", result)
            }
            Expression::BinOp {
                left: l,
                op: o,
                right: r,
            } => {
                write!(f, "{} {} {}", l, o, r)
            },
            Expression::Call {
                name: _,
                args: _,
            } => {
                unreachable!()
            }
            Expression::Nil => {
                write!(f, "")
            },
        }
    }
}

impl Expression {
    /// Simplify this expression, given a reference to a list of variables.
    pub fn simplify(&self, variables: &mut HashMap<String, Expression>) -> Self {
        match self {
            // Look up the variable and plug in
            Expression::Identifier (s) => {
                let expr = match variables.get(s) {
                    Some(e) => {
                        (*e).to_owned().simplify(variables)
                    },
                    None => {
                        self.to_owned()
                    },
                };

                expr
            },

            // Insert the assigned variable into the list of variables
            Expression::Assignment {
                identifier: ref i,
                value: ref v,
            } => {
                // Simplify the value of assignment
                let simplified = (**v).simplify(variables);

                // Register the variable
                variables.insert(i.to_owned(), simplified.to_owned());

                // Return the simplified value
                simplified.to_owned()
            }

            // Simplify the left and right and return
            Expression::BinOp {
                left: l,
                op: o,
                right: r,
            } => {
                // Simplify the left-hand and right-hand sides
                let left = l.simplify(variables);
                let right = r.simplify(variables);

                if let Expression::Int (l) = left {
                    if let Expression::Int (r) = right {
                        // Evaluate this as a float, then try to cast it to an `Int`
                        let f = binop(l as f64, r as f64, &o);
                        if f.fract() == 0.0 {
                            Expression::Int (f as i64)
                        } else {
                            Expression::Float (f)
                        }
                    } else if let Expression::Float (r) = right {
                        let left_float = l as f64;
                        Expression::Float (binop(left_float, r, &o))
                    } else if let Expression::Matrix {
                        rows: r,
                        cols: c,
                        values: v,
                    } = right {
                        let mut values = Vec::new();

                        for val in v {
                            values.push(Expression::BinOp {
                                left: Box::new(left.to_owned()),
                                op: "*".to_string(),
                                right: Box::new(val),
                            }.simplify(variables)); 
                        }

                        Expression::Matrix {
                            rows: r,
                            cols: c,
                            values,
                        }
                    } else {
                        Expression::BinOp {
                            left: Box::new(Expression::Int (l)),
                            op: o.to_owned(),
                            right: r.to_owned(),
                        }
                    }
                } else if let Expression::Float (l) = left {
                    if let Expression::Int (r) = right {
                        let right_float = r as f64;
                        Expression::Float (binop(l, right_float, &o))
                    } else if let Expression::Float (r) = right {
                        Expression::Float (binop(l, r, &o))
                    } else {
                        throw(InvalidOperands);
                        return Expression::Nil;
                    }
                } else if let Expression::Matrix {
                    rows: r,
                    cols: k1,
                    values: vl,
                } = left {
                    if let Expression::Matrix {
                        rows: k2,
                        cols: c,
                        values: vr,
                    } = right {
                        if k1 != k2 {
                            throw(ImproperDimensions);
                            return Expression::Nil;
                        }

                        matrix_dot(vl, vr, r, c, k1)
                    } else {
                        throw(InvalidOperands);
                        return Expression::Nil;
                    }
                } else {
                    return self.to_owned();
                }
            },
            
            // `Int is already in simplest form
            Expression::Int (_) => self.to_owned(),

            // `Float` can be reduced to `Int` if it has no fractional part
            Expression::Float (f) => {
                if f.fract() == 0.0 {
                    Expression::Int (*f as i64)
                } else {
                    Expression::Float (*f)
                }
            },
            
            // To simplify a `Matrix`, simplify each value
            Expression::Matrix {
                rows: r,
                cols: c,
                values: v,
            } => {
                let mut new = Vec::new();

                for val in v {
                    new.push(val.simplify(variables));
                }

                if *r == 1 && *c == 1 {
                    v[0].simplify(variables).to_owned()
                } else {
                    Expression::Matrix {
                        rows: *r,
                        cols: *c,
                        values: new,
                    }
                }
            },

            // To simplify a call, look up the function in the standard library
            // and pass the arguments necessary
            // 
            // In Elemental, all functions act on matrices.  
            Expression::Call {
                name: n,
                args: a,
            } => {
                // Simplify each argument and convert them to "native" matrices.
                let mut args = Vec::<Matrix>::new();
                for arg in a {
                    let simplified = arg.simplify(variables);
                    if let Expression::Matrix {
                        rows: r,
                        cols: c,
                        values: v,
                    } = simplified {
                        // Convert each value in the matrix from `Expression` to `f64`.
                        let mut values: Vec<f64> = Vec::new();
                        for value in v {
                            if let Self::Int (i) = value {
                                values.push(i as f64);
                            } else if let Self::Float (f) = value {
                                values.push(f);
                            } else {
                                // A value in one of the matrices is not a numeric literal
                                throw(InvalidValue);
                                return Expression::Nil;
                            }
                        }
                        args.push(Matrix::new(r, c, values));
                    } else if let Expression::Int (i) = simplified {
                        // If one value is a number, convert it into a 1x1 matrix
                        args.push(Matrix::new(1, 1, vec![i as f64]));
                    } else if let Expression::Float (f) = simplified {
                        // If one value is a number, convert it into a 1x1 matrix
                        args.push(Matrix::new(1, 1, vec![f]));
                    } else {
                        // One of the arguments is not a matrix or a number
                        throw(InvalidOperands);
                    }
                }

                let stdfn = get_std_function(n.to_owned());
                let output_matrix = stdfn.eval(args);

                let values = output_matrix.copy_vals().iter().map(|x| Self::Float (*x)).collect::<Vec<Self>>();

                Self::Matrix {
                    rows: output_matrix.rows(),
                    cols: output_matrix.cols(),
                    values,
                }.simplify(variables)
            },
            
            // `Nil` is already in simplest form
            Expression::Nil => self.to_owned(),
        }
    }
}


/// Executes the given binary operation on two floats.
pub fn binop(x: f64, y: f64, binop: &str) -> f64 {
    match binop {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => {
            if y == 0.0 {
                throw(DividedByZero);
                0.0
            } else {
                x / y
            }
        },
        _ => {
            throw(InvalidOperator);
            0.0
        },
    }
}


/// Computes the dot product of two matrices.
pub fn matrix_dot(left: Vec<Expression>, right: Vec<Expression>, rows: usize, cols: usize, count: usize) -> Expression {
    let mut values = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let mut cell = Expression::Int (0);
            for k in 0..count {
                // Add the addend to the cell
                let addend = Expression::BinOp {
                    left: Box::new(left[i*count + k].to_owned()),
                    right: Box::new(right[k*cols + j].to_owned()),
                    op: "*".to_string(),
                };

                cell = Expression::BinOp {
                    left: Box::new(cell),
                    right: Box::new(addend),
                    op: "+".to_string(),
                };
            }
            // Push the cell to the list of values
            values.push(cell.simplify(&mut HashMap::new()));
        }
    }

    Expression::Matrix {
        rows,
        cols,
        values,
    }
}