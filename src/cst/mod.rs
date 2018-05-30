//! The concrete syntax tree.

mod display;
mod grammar;

use symbol::Symbol;

use ast::Literal;

/// The full CST of a program.
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    /// The declarations in the program.
    pub decls: Vec<Decl>,

    /// The main expression.
    pub main: Expr,
}

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
    /// An addition.
    Add(Box<Expr>, Box<Expr>),

    /// A function call.
    Call(Box<Expr>, Box<Expr>),

    /// A consing.
    Cons(Box<Expr>, Box<Expr>),

    /// A division.
    Div(Box<Expr>, Box<Expr>),

    /// A list.
    ///
    /// In theory, this can't be empty (since that'd be the nil literal).
    /// Either way, can't hurt to handle all cases.
    List(Vec<Expr>),

    /// A literal value.
    Literal(Literal),

    /// A multiplication.
    Mul(Box<Expr>, Box<Expr>),

    /// A modulus/remainder.
    Mod(Box<Expr>, Box<Expr>),

    /// A subtraction.
    Sub(Box<Expr>, Box<Expr>),

    /// A variable.
    Variable(Symbol),
}
