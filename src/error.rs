//! Provides abstract error handling for the Elemental interpreter.

use colored::*;

/// Enumerates the types of errors available to the Elemental interpreter.
pub enum Error {
    CouldNotFlushOutput,
    CouldNotReadStdin,
    ImproperDimensions,
    InvalidOperands,
    InvalidOperator,
    InvalidValue,
    CouldNotParseNumeric,
    UnexpectedEof,
    CouldNotFindFunction,
    WrongNumberOfArgs,
    RequiresUnitMatrix,
    SquareMatrixRequired,
    ExpectedIdentifier,
    ExpectedCloseParen,
    DividedByZero,
    UndeclaredVariable (String),
    CouldNotReadFile (String),
    CouldNotWriteToFile,
    CouldNotDisplayPlot,
}

pub use Error::*;


/// Throws errors.
pub fn throw(error: Error) {
    let message: String = match error {
        CouldNotFlushOutput => "could not flush stdout".to_string(),
        CouldNotReadStdin => "could not read stdin".to_string(),
        ImproperDimensions => "improper dimensions".to_string(),
        InvalidOperands => "invalid binary operands".to_string(),
        InvalidOperator => "invalid operator".to_string(),
        InvalidValue => "at least one value in this matrix is not a numeric literal".to_string(),
        CouldNotParseNumeric => "could not parse numeric input".to_string(),
        UnexpectedEof => "unexpected token or end of token stream".to_string(),
        CouldNotFindFunction => "could not find function in standard library".to_string(),
        WrongNumberOfArgs => "wrong number of arguments passed to function".to_string(),
        RequiresUnitMatrix => "function requires a unit (1x1) matrix".to_string(),
        SquareMatrixRequired => "function requires a square matrix".to_string(),
        ExpectedIdentifier => "expected identifier".to_string(),
        ExpectedCloseParen => "expected closing parenthesis".to_string(),
        DividedByZero => "attempted to divide by zero".to_string(),
        UndeclaredVariable (s) => format!("found undeclared variable {}", s),
        CouldNotReadFile (s) => format!("could not read file {}", s),
        CouldNotWriteToFile => "unable to export data to file".to_string(),
        CouldNotDisplayPlot => "could not display plot in terminal".to_string(),
    };

    println!("{}: {}", "error".bold().red(), message);
}