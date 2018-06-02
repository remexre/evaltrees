//! The evaltrees parser, typechecker, and evaluator.
#![warn(missing_docs)]

#[macro_use]
extern crate failure;
extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;
extern crate linked_hash_set;
#[macro_use]
extern crate log;
extern crate regex;
extern crate symbol;

pub mod ast;
pub mod cst;
pub mod typeck;
