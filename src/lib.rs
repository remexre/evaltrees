//! The evaltrees parser, typechecker, and evaluator.
#![warn(missing_docs)]

#[macro_use]
extern crate display_attr;
extern crate either;
#[macro_use]
extern crate failure;
extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;
extern crate linked_hash_set;
#[macro_use]
extern crate log;
extern crate petgraph;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate regex;
extern crate symbol;

pub mod ast;
pub mod cst;
pub mod eval;
pub mod repl;
pub mod typeck;

#[cfg(test)]
mod tests;
