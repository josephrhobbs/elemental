//! The standard library for Elemental.
//! 
//! This library defines all built-in functions for the Elemental language.
//! It exports a `HashMap` to the main interpreter, allowing the interpreter
//! to connect function names to function definitions.

pub mod determinant;
pub mod transpose;
pub mod identity;
pub mod invert;
pub mod get_minors;

use std::collections::HashMap;

use crate::Matrix;

pub use determinant::{
    determinant,
    detf64,
};
pub use transpose::transpose;
pub use identity::identity;
pub use invert::invert;
pub use get_minors::get_minors;


/// Get a function pointer based on that function's name.
pub fn get_std_function(name: String) -> impl Fn(Vec<Matrix>) -> Matrix {
    let mut hashmap: HashMap<String, fn(&Matrix) -> Matrix> = HashMap::new();

    // Declarative standard library begins here
    hashmap.insert("t".to_string(), transpose);
    hashmap.insert("det".to_string(), determinant);
    hashmap.insert("I".to_string(), identity);
    hashmap.insert("inv".to_string(), invert);

    match hashmap.get(&name) {
        Some(f) => wrap(*f),
        None => todo!(),
    }
}


/// Wraps a `fn(&Matrix) -> Matrix` with a `fn(Vec<Matrix>) -> Matrix`.  Facilitates compatibility with Elemental's standard library.
fn wrap(func: fn(&Matrix) -> Matrix) -> impl Fn(Vec<Matrix>) -> Matrix {
    move |vec| {
        if vec.len() != 1 {
            todo!();
        }

        func(&vec[0])
    }
}