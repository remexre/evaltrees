//! The evaltrees parser, typechecker, and evaluator.
#![warn(missing_docs)]

extern crate lalrpop_util;
#[macro_use]
extern crate log;
extern crate regex;
extern crate symbol;

pub mod ast;
pub mod cst;
