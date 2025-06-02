//! The evaltrees parser, typechecker, and evaluator.
#![warn(missing_docs)]

pub mod ast;
pub mod cst;
pub mod eval;
pub mod repl;
pub mod typeck;

pub use crate::eval::EvalError;
pub use crate::typeck::TypeError;
pub use crate::ast::ASTConversionError; // Now correctly pathed via ast::mod.rs

#[cfg(test)]
mod tests;
