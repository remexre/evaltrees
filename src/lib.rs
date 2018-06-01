//! The evaltrees parser, typechecker, and evaluator.
#![warn(missing_docs)]

extern crate hamt_rs;
extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;
extern crate symbol;

pub mod ast;
pub mod cst;
pub mod typeck;
pub mod typed_ast;
