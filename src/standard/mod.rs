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

use std::{
    collections::HashMap,
    rc::Rc,
};

use crate::Matrix;

pub use determinant::Determinant;
pub use transpose::Transpose;
pub use identity::Identity;
pub use invert::Invert;
pub use get_minors::GetMinors;


/// Any function available in the standard library satisfies this trait.
pub trait StdFunc {
    fn eval(&self, args: Vec<Matrix>) -> Matrix;
}


/// Get a function pointer based on that function's name.
pub fn get_std_function(name: String) -> Rc<dyn StdFunc> {
    let mut hashmap: HashMap<String, Rc<dyn StdFunc>> = HashMap::new();

    // Declarative standard library begins here
    hashmap.insert("t".to_string(), Rc::new(Transpose {}));
    hashmap.insert("det".to_string(), Rc::new(Determinant {}));
    hashmap.insert("I".to_string(), Rc::new(Identity {}));
    hashmap.insert("inv".to_string(), Rc::new(Invert {}));

    match hashmap.get(&name) {
        Some(f) => f.clone(),
        None => todo!(),
    }
}