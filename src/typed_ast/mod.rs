//! A version of the abstract syntax tree with types.

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

    /// The type of the declared name.
    pub ty: Type,
}

/// A pattern.
#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    /// A name.
    Binding(Symbol, Type),

    /// A cons.
    Cons(Box<Pattern>, Box<Pattern>, Type),

    /// A literal value.
    Literal(Literal),
}

/// An expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A literal value.
    Literal(Literal),

    /// A binary operator.
    Op(Op, Box<Expr>, Box<Expr>, Type),

    /// A variable.
    Variable(Symbol, Type),
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

impl Literal {
    /// Returns the type of the literal.
    pub fn ty(self) -> Type {
        match self {
            Literal::Int(_) => Type::Int,
            Literal::Nil => Type::Forall(Box::new(Type::List(Box::new(Type::Var(0))))),
        }
    }
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
