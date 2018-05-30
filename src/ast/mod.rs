//! The abstract syntax tree.

mod convert;
mod display;

use symbol::Symbol;

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

/// A pattern.
#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    /// A name.
    Binding(Symbol),

    /// A cons.
    Cons(Box<Pattern>, Box<Pattern>),

    /// A literal value.
    Literal(Literal),
}

/// An expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A literal value.
    Literal(Literal),

    /// A binary operator.
    Op(Op, Box<Expr>, Box<Expr>),

    /// A variable.
    Variable(Symbol),
}

/// A binary operator.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Op {
    /// Addition.
    Add,

    /// Function application.
    App,

    /// Consing.
    Cons,

    /// Division.
    Div,

    /// Modulus.
    Mod,

    /// Multiplication.
    Mul,

    /// Subtraction.
    Sub,
}

/// A literal value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    /// An (unsigned) integer.
    Int(usize),

    /// An empty list.
    Nil,
}
