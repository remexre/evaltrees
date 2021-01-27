//! The evaltrees parser, typechecker, and evaluator.
#![warn(missing_docs)]

pub mod ast;
pub mod cst;
pub mod eval;
pub mod repl;
pub mod typeck;

#[cfg(test)]
mod tests;
