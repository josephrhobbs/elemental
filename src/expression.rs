//! Abstracts over Elemental expressions.

use std::{
    fmt::{
        Display,
        Result,
        Formatter,
    },
    collections::HashMap,
};

use colored::*;

use crate::standard::get_std_function;

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
                                "{:^8}",
                                format!("{}", v[index as usize])
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
                todo!()
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
                    Some(e) => (*e).to_owned(),
                    None => {
                        print!("{}: variable {} has not been declared", "error".red().bold(), s);
                        return Expression::Nil;
                    },
                };
                // Simplify
                expr.simplify(variables)
            },

            // Insert the assigned variable into the list of variables
            Expression::Assignment {
                identifier: ref i,
                value: ref v,
            } => {
                // Register the variable
                variables.insert(i.to_owned(), *v.to_owned());

                // Simplify the value of assignment and return it
                (**v).simplify(variables).to_owned()
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
                        Expression::Int (binop_int(l, r, &o))
                    } else if let Expression::Float (r) = right {
                        let left_float = l as f64;
                        Expression::Float (binop_float(left_float, r, &o))
                    } else {
                        todo!()
                    }
                } else if let Expression::Float (l) = left {
                    if let Expression::Int (r) = right {
                        let right_float = r as f64;
                        Expression::Float (binop_float(l, right_float, &o))
                    } else if let Expression::Float (r) = right {
                        Expression::Float (binop_float(l, r, &o))
                    } else {
                        todo!()
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
                            todo!()
                        }

                        matrix_dot(vl, vr, r, c, k1)
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            },
            
            // `Int` and `Float` are both already in simplest form
            Expression::Int (_) => self.to_owned(),
            Expression::Float (_) => self.to_owned(),
            
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

                Expression::Matrix {
                    rows: *r,
                    cols: *c,
                    values: new,
                }
            },

            // To simplify a call, look up the function in the standard library
            // and pass the arguments necessary
            Expression::Call {
                name: n,
                args: a,
            } => {
                let stdfn: fn(Vec<Expression>) -> Expression = get_std_function(n.to_owned());

                // Simplify each value in `a` before passing to the function
                let mut args = Vec::new();
                for arg in a {
                    args.push(arg.simplify(variables));
                }

                stdfn(args).simplify(variables)
            },
            
            // `Nil` is already in simplest form
            Expression::Nil => self.to_owned(),
        }
    }
}


/// Executes the given binary operation on two integers.
pub fn binop_int(x: i64, y: i64, binop: &str) -> i64 {
    match binop {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => todo!(),
    }
}


/// Executes the given binary operation on two floats.
pub fn binop_float(x: f64, y: f64, binop: &str) -> f64 {
    match binop {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => todo!(),
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