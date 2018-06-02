//! The abstract syntax tree.

mod convert;
mod display;

use symbol::Symbol;

/// A function or value declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Decl<Aux = ()> {
    /// The name of the function or value.
    pub name: Symbol,

    /// The arguments to the function. If empty, the decl is for a value.
    pub args: Vec<Pattern<Aux>>,

    /// The body of the function, or the expression assigned to the value.
    pub body: Expr<Aux>,

    /// Auxiliary data.
    pub aux: Aux,
}

impl<Aux> Decl<Aux> {
    /// Gets the auxiliary data.
    pub fn get_aux(&self) -> &Aux {
        &self.aux
    }
}

/// A pattern.
#[derive(Clone, Debug, PartialEq)]
pub enum Pattern<Aux = ()> {
    /// A name.
    Binding(Symbol, Aux),

    /// A cons.
    Cons(Box<Pattern<Aux>>, Box<Pattern<Aux>>, Aux),

    /// A literal value.
    Literal(Literal, Aux),
}

impl<Aux> Pattern<Aux> {
    /// Gets the auxiliary data.
    pub fn get_aux(&self) -> &Aux {
        match *self {
            Pattern::Binding(_, ref aux)
            | Pattern::Cons(_, _, ref aux)
            | Pattern::Literal(_, ref aux) => aux,
        }
    }
}

/// An expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr<Aux = ()> {
    /// A literal value.
    Literal(Literal, Aux),

    /// A binary operator.
    Op(Op, Box<Expr<Aux>>, Box<Expr<Aux>>, Aux),

    /// A variable.
    Variable(Symbol, Aux),
}

impl<Aux> Expr<Aux> {
    /// Gets the auxiliary data.
    pub fn get_aux(&self) -> &Aux {
        match *self {
            Expr::Literal(_, ref aux) | Expr::Op(_, _, _, ref aux) | Expr::Variable(_, ref aux) => {
                aux
            }
        }
    }
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

/// A (fully formed) type.
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    /// Universal quantification over a variable.
    ///
    /// De Brujin indices are used here, so no explicit names are needed.
    Forall(Box<Type>),

    /// A function type.
    Func(Box<Type>, Box<Type>),

    /// An integral type.
    Int,

    /// A list type.
    List(Box<Type>),

    /// A type variable.
    Var(usize),
}
