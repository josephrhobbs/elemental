//! The standard library for Elemental.
//! 
//! This library defines all built-in functions for the Elemental language.
//! It exports a `HashMap` to the main interpreter, allowing the interpreter
//! to connect function names to function definitions.

mod determinant;
mod transpose;
mod identity;
mod invert;
mod get_minors;
mod sqrt;
mod sin;
mod cos;
mod exit;
mod cross;
mod dot;
mod linspace;
mod plot;

use std::{
    collections::HashMap,
    rc::Rc,
};

use crate::Matrix;
use crate::error::*;

pub use determinant::Determinant;
pub use transpose::Transpose;
pub use identity::Identity;
pub use invert::Invert;
pub use get_minors::GetMinors;
pub use sqrt::Sqrt;
pub use sin::Sin;
pub use cos::Cos;
pub use exit::Exit;
pub use cross::Cross;
pub use dot::Dot;
pub use linspace::Linspace;
pub use plot::Plt;

/// Any function available in the standard library satisfies this trait.
pub trait StdFunc {
    fn eval(&self, args: Vec<Matrix>) -> Matrix;
}


/// A unit struct passed by `get_std_function` when a function is not found.
/// 
/// This allows the interpreter to continue working without panicking.
pub struct Error;

impl StdFunc for Error {
    fn eval(&self, _args: Vec<Matrix>) -> Matrix {
        Matrix::new(0, 0, Vec::new())
    }
}


/// Get a function pointer based on that function's name.
pub fn get_std_function(name: String) -> Rc<dyn StdFunc> {
    let mut hashmap: HashMap<String, Rc<dyn StdFunc>> = HashMap::new();

    // Declarative standard library begins here
    hashmap.insert("t".to_string(), Rc::new(Transpose {}));
    hashmap.insert("det".to_string(), Rc::new(Determinant {}));
    hashmap.insert("I".to_string(), Rc::new(Identity {}));
    hashmap.insert("inv".to_string(), Rc::new(Invert {}));
    hashmap.insert("sqrt".to_string(), Rc::new(Sqrt {}));
    hashmap.insert("sin".to_string(), Rc::new(Sin {}));
    hashmap.insert("cos".to_string(), Rc::new(Cos {}));
    hashmap.insert("exit".to_string(), Rc::new(Exit {}));
    hashmap.insert("cross".to_string(), Rc::new(Cross {}));
    hashmap.insert("dot".to_string(), Rc::new(Dot {}));
    hashmap.insert("linspace".to_string(), Rc::new(Linspace {}));
    hashmap.insert("plot".to_string(), Rc::new(Plt {}));

    match hashmap.get(&name) {
        Some(f) => f.clone(),
        None => {
            throw(CouldNotFindFunction);
            Rc::new(Error {})
        },
    }
}