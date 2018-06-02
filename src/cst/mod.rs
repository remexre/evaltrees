//! The concrete syntax tree.

mod display;
mod parser;

use symbol::Symbol;

use ast::{Literal, Op, Pattern};
pub use cst::parser::parse_decls;

/// A function or value declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Decl {
    /// The name of the function or value.
    pub name: Symbol,

    /// The arguments to the function. If empty, the decl is for a value.
    pub args: Vec<Pattern>,

    /// The body of the function, or the expression assigned to the value.
    pub body: Expr,
}

/// An expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A list.
    ///
    /// In theory, this can't be empty (since that'd be the nil literal).
    /// Either way, can't hurt to handle all cases.
    List(Vec<Expr>),

    /// A literal value.
    Literal(Literal),

    /// A binary operation.
    Op(Op, Box<Expr>, Box<Expr>),

    /// A variable.
    Variable(Symbol),
}
