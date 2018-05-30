//! The abstract syntax tree.

mod display;

use symbol::Symbol;

/// The full AST of a program.
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
    /// A binary operator.
    Binop(Binop, Box<Expr>, Box<Expr>),

    /// A function call.
    Call(Box<Expr>, Vec<Expr>),

    /// A literal value.
    Literal(Literal),

    /// A variable.
    Variable(Symbol),
}

/// A binary operator.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Cons,
}

/// A literal value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    /// An (unsigned) integer.
    Int(usize),

    /// An empty list.
    Nil,
}
